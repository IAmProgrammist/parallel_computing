#include <stdio.h>
#include <omp.h>
#include <math.h>
#include <stdlib.h>

#define NUM_ITERATIONS 10000

long double factorial(const int n)
{
    long double f = 1;
    for (int i = 1; i <= n; ++i)
        f *= i;
    return f;
}

int main()
{
    long double *sums = malloc(NUM_ITERATIONS * sizeof(long double));
    long double sum = 0.0;

#pragma omp parallel
    {
#pragma omp for schedule(guided)
        for (int i = 0; i < NUM_ITERATIONS; i++)
        {
            sums[i] = (cos(pow(i, 3)) + pow(i, 4) * exp(-i) + log(i + 1)) /
                      (sqrt(i * i + tan(i) + 1) + factorial(i + 1));
        }

#pragma omp for reduction(+ : sum)
        for (int i = 0; i < NUM_ITERATIONS; i++)
        {
            sum += sums[i];
        }
    }

    free(sums);

    return 0;
}