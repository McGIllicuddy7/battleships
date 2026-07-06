make: src/main.c bridget-libc/blibc.c
	gcc src/main.c bridget-libc/blibc.c -Wall -Wextra -std=c99 -pedantic -Wmissing-prototypes -Wstrict-prototypes -Wold-style-definition -g3 -fsanitize=address