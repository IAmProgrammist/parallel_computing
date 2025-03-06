# Компилятор
CC=gcc

# Имя исполняемого файла
EXECUTABLE=lab2


# Основная задача
all: 
	$(CC) -fopenmp lab2_equal.c -o $(EXECUTABLE)_equal -lm
	$(CC) -fopenmp lab2_non_equal.c -o $(EXECUTABLE)_non_equal -lm

clean:
	rm -rf *.o $(EXECUTABLE)