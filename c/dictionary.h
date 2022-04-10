#include <stdbool.h>

#include "string.h"

typedef struct {
    string_t entries[4096];
    int length;
} dictionary_t;

void dictionary_init(dictionary_t *);
bool dictionary_contains(dictionary_t *, string_t *);
int dictionary_get_code(dictionary_t *, string_t *);
/* 
 * Note that adding a string to a dictionary transfers ownership of that string
 * to the dictionary. Hence the dictionary will be responsible for cleaning up
 * the string and it is an error for clients to clean up the string's 
 * underlying resources.
 */
void dictionary_add(dictionary_t *, string_t);
void dictionary_deinit(dictionary_t *);
