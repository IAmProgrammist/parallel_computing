#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <mpi.h>
#include <omp.h>
#include <math.h>

#define N 80000000

void classic_sieve(int* sieve, int sieve_size) {
    for (int i = 2; i < sieve_size; i++) {
        if (sieve[i]) continue;
        for (int j = 2 * i; j < N; j += i)
            sieve[j] = true; 
    }
}

int main(int argc, char* argv[]) {
    // Инициализировать MPI, получить ранг 
    // и количество процессов MPI
    int rank, size;

    // Просчёт локального решета Эратосфена
    int* local_sieve = (int*) calloc(N, sizeof(int));
    // Просчитываем первые sqrt(N) чисел
    classic_sieve(local_sieve, sqrt(N));

    MPI_Init(&argc, &argv);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &size);

    // Определяем границы для которых будет вычисляться локальное решето
    int left = sqrt(N) + ((N - sqrt(N)) / size) * rank;
    int right = rank == size - 1 ? N : (sqrt(N) + ((N - sqrt(N)) / size) * (rank + 1));

    // Проходим по всем простым числам до sqrt(N)
    for (int i = 2; i < sqrt(N); i++) {
        if (local_sieve[i]) continue;

        // Находим первое число кратное i в рамках определённых границ
        int j = left / i * i;
        if (j < left) {
            j += i;
        }

        // Устанавливаем кратные числа
        #pragma omp parallel for schedule(dynamic)
        for (int jj = j; jj < right; jj += i) {
            local_sieve[jj] = true;
        }
    }

    int* global_sieve = (int*) calloc(N, sizeof(int));

    MPI_Reduce(
        local_sieve,
        global_sieve,
        N,
        MPI_INT,
        MPI_LOR,
        0,
        MPI_COMM_WORLD
    );
    
    // Очистить ресурсы MPI
    MPI_Finalize();
    return 0;
}
