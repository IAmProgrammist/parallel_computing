#define _GNU_SOURCE
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <sched.h>
#include <unistd.h>
#include <stdint.h>
#include <math.h>

#ifndef NUM_THREADS
#define NUM_THREADS 4
#endif

#define NUM_ITERATIONS 10000000

volatile long double sums[NUM_THREADS];

long double factorial(const long double n)
{
    long double f = 1;
    for (long double i = 1; i <= n; ++i)
        f *= i;
    return f;
}

void *compute(void *arg)
{
    size_t iteration_num = (size_t)arg;
    size_t lower_bound = (NUM_ITERATIONS / NUM_THREADS) * iteration_num + 1;
    size_t upper_bound = (NUM_ITERATIONS / NUM_THREADS) * (iteration_num + 1) + 1;

    long double iter_factorial = factorial(lower_bound);

    for (uint64_t iter = lower_bound; iter < upper_bound; iter++)
    {
        sums[iteration_num] += (cos(pow(iter, 3)) + pow(iter, 4) * exp(-iter) + log(iter + 1)) /
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
    {
        sums[iter] = 0;
        if (mode == 1)
        {
            pthread_create(&threads[iter], NULL, compute, (void *)iter);
        }
        else
        {
            pthread_create(&threads[iter], NULL, compute_pinned, (void *)iter);
        }
    }
    for (size_t iter = 0; iter < NUM_THREADS; iter++)
        pthread_join(threads[iter], NULL);
    
    long double sum = 0;
    for (int i = 0; i < NUM_THREADS; i++) {
        sum += sums[i];
    }

    return EXIT_SUCCESS;
}