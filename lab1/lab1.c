#define _GNU_SOURCE
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <sched.h>
#include <unistd.h>
#include <stdint.h>
#include <math.h>
#define NUM_THREADS 4
#define NUM_ITERATIONS 10000000

void *compute(void *arg)
{
    volatile long double sum = 0;
    long double iter_factorial = 1;
    for (uint64_t iter = 0; iter < NUM_ITERATIONS; iter++)
    {
        sum += (cos(pow(iter, 3)) + pow(iter, 4) * exp(-iter) + log(iter + 1)) /
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

void *compute_pinned(void *arg)
{
    int core_id = (int)(long)arg;
    pin_thread_to_core(core_id);
    return compute(arg);
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
    for (size_t iter = 0; iter < NUM_THREADS; iter++)
        if (mode == 1)
            pthread_create(&threads[iter], NULL, compute, (void *)iter);
        else
            pthread_create(&threads[iter], NULL, compute_pinned, (void *)iter);
    for (size_t iter = 0; iter < NUM_THREADS; iter++)
        pthread_join(threads[iter], NULL);
    return EXIT_SUCCESS;
}