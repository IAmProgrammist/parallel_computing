#include <stdio.h>
#include <omp.h>
#include <math.h>
#include <stdlib.h>

#define NUM_ITERATIONS 10000000

int main()
{
    // Непараллельные вычисления факториалов
    long double *factorials = malloc(NUM_ITERATIONS * sizeof(long double));
    factorials[0] = 1;

    for (int i = 1; i < NUM_ITERATIONS; i++)
    {
        factorials[i] = factorials[i - 1] * (i + 1.0);
    }

    long double *sums = malloc(NUM_ITERATIONS * sizeof(long double));
    long double sum = 0.0;

#pragma omp parallel
    {
#pragma omp for schedule(auto)
        for (int i = 0; i < NUM_ITERATIONS; i++)
        {
            sums[i] = (cos(pow(i, 3)) + pow(i, 4) * exp(-i) + log(i + 1)) /
                      (sqrt(i * i + tan(i) + 1) + factorials[i]);
        }

#pragma omp for reduction(+ : sum)
        for (int i = 0; i < NUM_ITERATIONS; i++)
        {
            sum += sums[i];
        }
    }

    free(factorials);
    free(sums);

    return 0;
}