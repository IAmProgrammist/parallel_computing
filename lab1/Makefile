# Компилятор
CC=gcc

# Флаги компиляции
CFLAGS=-c -Wall

# Флаги линковки
LDFLAGS=-pthread

# Оверрайд переменных компиляции
DFLAGS="-DNUM_THREADS=2"

# Имя исполняемого файла
EXECUTABLE=lab1

# Исходники
SOURCES=lab1.c

# Объектные файлы
OBJECTS=$(SOURCES:.c=.o)


# Основная задача
all: $(SOURCES) $(EXECUTABLE)
	
# Задача по линковке. Линкует все .o файлы в один
$(EXECUTABLE): $(OBJECTS) 
	$(CC) $(LDFLAGS) $(OBJECTS) -o $@ -lm

# Задача по компиляции, компилирует все .c файлы в .o
%.o : %.c
	$(CC) $(DFLAGS) -c $(CFLAGS) $< -o $@

clean:
	rm -rf *.o $(EXECUTABLE)