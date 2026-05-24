#include "../include/string_utils.h"
#include <stdio.h>
#include <ctype.h>

void print_upper(const char *s) {
  while (*s) putchar(toupper(*s++));
  putchar ('\n');

}
