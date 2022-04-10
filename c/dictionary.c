#include <stdlib.h>
#include <string.h>

#include "dictionary.h"

void dictionary_init(dictionary_t *dictionary)
{
    for (int i = 0; i < 256; i++) {
        string_init(&dictionary->entries[i]);
        string_append(&dictionary->entries[i], i);
    }

    dictionary->length = 256;
}

bool dictionary_contains(dictionary_t *dictionary, string_t *string)
{
    for (int i = 0; i < dictionary->length; i++) {
        if (string_equals(&dictionary->entries[i], string)) {
            return true;
        }
    }

    return false;
}

int dictionary_get_code(dictionary_t *dictionary, string_t *string)
{
    for (int i = 0; i < dictionary->length; i++) {
        if (string_equals(&dictionary->entries[i], string)) {
            return i;
        }
    }

    return -1;
}

void dictionary_add(dictionary_t *dictionary, string_t string)
{
    dictionary->entries[dictionary->length] = string;
    dictionary->length += 1;
    // TODO: reset dictionary
}

void dictionary_deinit(dictionary_t *dictionary)
{
    for (int i = 0; i < dictionary->length; i++) {
        string_deinit(&dictionary->entries[i]);
    }
}
