#include <stdbool.h>

typedef struct {
    char *string;
    int length;
    int offset;
} string_t;

void string_init(string_t *);
void string_append(string_t *, char);
bool string_equals(string_t *, string_t *);
/*
 * A string slice points to the same underlying string as the input. Modifying
 * a slice will modify the original string and vice-versa. The slice should
 * be treated as a read-only copy.
 */
string_t string_slice(string_t *, int offset, int length);
void string_deinit(string_t *);
