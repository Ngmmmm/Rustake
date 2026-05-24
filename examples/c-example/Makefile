CC = gcc
CFLAGS = -Wall -Iinclude

program: src/main.c src/math_utils.c src/string_utils.c
	$(CC) $(CFLAGS) $^ -o program

.PHONY: clean
clean:
	rm -f program
