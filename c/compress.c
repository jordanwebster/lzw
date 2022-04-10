#include <stdio.h>

#include "buffer.h"
#include "dictionary.h"

void compress(FILE *input, FILE *output) {
    dictionary_t dictionary;
    dictionary_init(&dictionary);

    string_t string;
    string_init(&string);

    buffer_t buffer;
    buffer_init(&buffer, output);

    char c;
    while ((c=getc(input)) != EOF) {
        string_append(&string, c);
        if (!dictionary_contains(&dictionary, &string)) {
            string_t substring = string_slice(&string, 0, string.length - 1);
            buffer_write(&buffer, dictionary_get_code(&dictionary, &substring));

            dictionary_add(&dictionary, string);
            string_init(&string);
            string_append(&string, c);
        }
    }

    buffer_flush(&buffer);
    
    dictionary_deinit(&dictionary);
    string_deinit(&string);
}
