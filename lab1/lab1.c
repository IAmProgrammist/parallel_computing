#define _GNU_SOURCE
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <sched.h>
#include <unistd.h>
#include <stdint.h>
#include <math.h>
#define NUM_THREADS 3
#define NUM_ITERATIONS 10000000

volatile long double component_a = 0;
volatile long double component_b = 0;
volatile long double component_c = 0;

typedef void *(*ComponentComputer)(void *arg);

void *compute_component_a(void *arg)
{
    long double iter_factorial = 1;
    for (uint64_t iter = 0; iter < NUM_ITERATIONS; iter++)
    {
        component_a += (cos(pow(iter, 3))) /
                       (sqrt(iter * iter + tan(iter) + 1) + iter_factorial);
        iter_factorial *= iter;
    }
}

void *compute_component_b(void *arg)
{
    long double iter_factorial = 1;
    for (uint64_t iter = 0; iter < NUM_ITERATIONS; iter++)
    {
        component_b += (pow(iter, 4) * exp(-iter)) /
                       (sqrt(iter * iter + tan(iter) + 1) + iter_factorial);
        iter_factorial *= iter;
    }
}

void *compute_component_c(void *arg)
{
    long double iter_factorial = 1;
    for (uint64_t iter = 0; iter < NUM_ITERATIONS; iter++)
    {
        component_c += (log(iter + 1)) /
                       (sqrt(iter * iter + tan(iter) + 1) + iter_factorial);
        iter_factorial *= iter;
    }
}

// Принудительное закрепление потока за конкретным ядром
void pin_thread_to_core(int core_id)
{
    cpu_set_t cpuset;
    CPU_ZERO(&cpuset);
    CPU_SET(core_id, &cpuset);
    pthread_t current_thread = pthread_self();
    pthread_setaffinity_np(current_thread, sizeof(cpu_set_t), &cpuset);
}

void *compute_pinned_a(void *arg)
{
    int core_id = (int)(long)arg;
    pin_thread_to_core(core_id);
    return compute_component_a(arg);
}

void *compute_pinned_b(void *arg)
{
    int core_id = (int)(long)arg;
    pin_thread_to_core(core_id);
    return compute_component_b(arg);
}

void *compute_pinned_c(void *arg)
{
    int core_id = (int)(long)arg;
    pin_thread_to_core(core_id);
    return compute_component_c(arg);
}

int main(int argc, char *argv[])
{
    pthread_t threads[NUM_THREADS];
    if (argc < 2)
    {
        fprintf(stderr, "Запуск: %s <mode>\n"
                        "Опции:\n 1 - конкурентность, одно ядро\n"
                        "2 - параллелизм, разные ядра\n",
                argv[0]);
        return EXIT_FAILURE;
    }
    int mode = atoi(argv[1]);
    printf("Старт: %s, %d поток(а)(ов)...\n",
           mode == 1 ? "конкурентность, одно ядро" : "параллелизм, разные ядра", NUM_THREADS);

    if (mode == 1)
    {
        pthread_create(&threads[0], NULL, compute_component_a, (void *)0);
        pthread_create(&threads[1], NULL, compute_component_b, (void *)1);
        pthread_create(&threads[2], NULL, compute_component_c, (void *)2);
    }
    else
    {
        pthread_create(&threads[0], NULL, compute_pinned_a, (void *)0);
        pthread_create(&threads[1], NULL, compute_pinned_b, (void *)1);
        pthread_create(&threads[2], NULL, compute_pinned_c, (void *)2);
    }

    for (size_t iter = 0; iter < NUM_THREADS; iter++)
        pthread_join(threads[iter], NULL);
    
    long double sum = component_a + component_b + component_c;

    return EXIT_SUCCESS;
}