#include <stddef.h>

int strcmp(const char *s1, const char *s2);
size_t strcspn(const char *s1, const char *s2);
size_t strlen(const char *str);
int strncmp(const char *s1, const char *s2, size_t n);
char * strrchr(const char *p, int ch);

#define DEF_STRONG(x)
#define __weak_alias(x, y)
