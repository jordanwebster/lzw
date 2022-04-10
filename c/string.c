#include <stdlib.h>
#include <string.h>

#include "string.h"

void string_init(string_t *string)
{
    string->string = calloc(4096, sizeof(char));
    string->length = 0;
    string->offset = 0;
}

void string_append(string_t *string, char c)
{
    string->string[string->length] = c;
    string->length += 1;
}

bool string_equals(string_t *lhs, string_t *rhs)
{
    return (
        lhs->length == rhs->length &&
        memcmp(&lhs->string[lhs->offset], &rhs->string[rhs->offset], lhs->length) == 0
    );
}

string_t string_slice(string_t *string, int offset, int length)
{
    string_t new_string = {
        .string = string->string,
        .length = length,
        .offset = string->offset + offset
    };
    return new_string;
}

void string_deinit(string_t *string)
{
    free(string->string);
}
