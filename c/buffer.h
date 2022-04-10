#include <stdbool.h>
#include <stdio.h>

typedef struct {
    FILE *output;
    char buf[3];
    bool is_first_code;
} buffer_t;

void buffer_init(buffer_t *, FILE *);
void buffer_write(buffer_t *, int);
void buffer_flush(buffer_t *);
