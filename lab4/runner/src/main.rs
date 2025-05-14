// FIXME(eddyb) update/review these lints.
//
// BEGIN - Embark standard lints v0.4
// do not change or add/remove here, but one can add exceptions after this section
// for more info see: <https://github.com/EmbarkStudios/rust-ecosystem/issues/59>
//#![deny(unsafe_code)] // impractical in this crate dealing with unsafe `ash`
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
// END - Embark standard lints v0.4
// crate-specific exceptions:
// #![allow()]

use clap::Parser;
use opencl3::error_codes::ClError;

use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::path::PathBuf;

use spirv_builder::{MetadataPrintout, SpirvBuilder};

// OpenCL libs
use opencl3::Result;
use opencl3::command_queue::{CL_QUEUE_PROFILING_ENABLE, CommandQueue};
use opencl3::context::Context;
use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_ALL, CL_DEVICE_TYPE_GPU};
use opencl3::kernel::{ExecuteKernel, Kernel};
use opencl3::memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use opencl3::program::Program;
use opencl3::types::{CL_BLOCKING, CL_NON_BLOCKING, cl_event, cl_float};
use std::ptr;

#[derive(Debug, Parser)]
#[command()]
pub struct Options {
    /// Use Vulkan debug layer (requires Vulkan SDK installed)
    #[arg(short, long)]
    debug_layer: bool,
}

pub struct OpenCLBinaries {
    pub program: Program,
    pub kernel: Kernel,
}

pub fn main() {
    // Find a usable device for this application
    let device_id = *get_all_devices(CL_DEVICE_TYPE_ALL)
        .expect("No available devices found")
        .first()
        .expect("No available devices found");
    let device = Device::new(device_id);

    // Create a Context on an OpenCL device
    let context = Context::from_device(&device).expect("Context::from_device failed");

    // Create a command_queue on the Context's device
    let queue = CommandQueue::create_default(&context, CL_QUEUE_PROFILING_ENABLE)
        .expect("CommandQueue::create_default failed");


    // Translate shaders into SPIR-V
    let shaders: Vec<OpenCLBinaries> = compile_shaders()
        .iter()
        .map(|file| -> OpenCLBinaries {
            let mut f = File::open(file.data.to_str().unwrap())
            .expect("Couln't open SPIR-V binary");
            let mut spirv_shader = Vec::new();

            f.read_to_end(&mut spirv_shader)
                .expect("Reading SPIR-V binary failed");

            // Build the OpenCL program source and create the kernel.
            let program = Program::create_and_build_from_il(&context,&spirv_shader[..], "")
                .expect("Program::create_and_build_from_source failed");
            let kernel = Kernel::create(&program, file.name.as_str())
            .expect("Kernel::create failed");

            OpenCLBinaries {
                program: program,
                kernel: kernel,
            }
        })
        .collect();
}

pub fn compile_shaders() -> Vec<SpvFile> {
    // Hack: spirv_builder builds into a custom directory if running under cargo, to not
    // deadlock, and the default target directory if not. However, packages like `proc-macro2`
    // have different configurations when being built here vs. when building
    // rustc_codegen_spirv normally, so we *want* to build into a separate target directory, to
    // not have to rebuild half the crate graph every time we run. So, pretend we're running
    // under cargo by setting these environment variables.
    std::env::set_var("OUT_DIR", env!("OUT_DIR"));
    std::env::set_var("PROFILE", env!("PROFILE"));

    SpirvBuilder::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/../shader"),
        "spirv-unknown-vulkan1.1",
    )
    .print_metadata(MetadataPrintout::None)
    .shader_panic_strategy(spirv_builder::ShaderPanicStrategy::DebugPrintfThenExit {
        print_inputs: true,
        print_backtrace: true,
    })
    // HACK(eddyb) needed because of `debugPrintf` instrumentation limitations
    // (see https://github.com/KhronosGroup/SPIRV-Tools/issues/4892).
    .multimodule(true)
    .build()
    .unwrap()
    .module
    .unwrap_multi()
    .iter()
    .map(|(name, path)| -> SpvFile {
        println!("{} {}", name, path.to_str().unwrap());

        return SpvFile {
            name: name.clone(),
            data: path.to_path_buf(),
        };
    })
    .collect()
}

#[derive(Debug)]
pub struct SpvFile {
    pub name: String,
    pub data: PathBuf,
}