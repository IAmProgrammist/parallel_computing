#!/bin/bash

echo "Компиляция..."
echo ""

MAX_THREADS=10

make clean
for i in $(seq 1 $MAX_THREADS); do 
    make DFLAGS="-DNUM_THREADS=$i";
    mv ./lab1 ./lab1_$((i))_threads
    make clean
done

echo "Запуск конкуренции: "
echo ""

for i in $(seq 1 $MAX_THREADS); do 
    time taskset -c 0 ./lab1_$((i))_threads 1
done

echo "Запуск параллелизма: "
echo ""

for i in $(seq 1 $MAX_THREADS); do 
    time ./lab1_$((i))_threads 2
done

echo "Очистка: "
echo ""

for i in $(seq 1 $MAX_THREADS); do 
    rm ./lab1_$((i))_threads
done