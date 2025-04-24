# Компилятор
CC=mpicxx

# Имя исполняемого файла
EXECUTABLE=lab3


# Основная задача
all: 
	$(CC) -fopenmp lab3mt.c -o $(EXECUTABLE)mt -lm

clean:
	rm -rf *.o $(EXECUTABLE)mt