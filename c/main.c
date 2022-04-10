#include <stdio.h>

#include "compress.h"

int main(int argc, char *argv[]) {
    FILE *input = fopen(argv[1], "rb");
    FILE *output = fopen(argv[2], "wb");

    compress(input, output);

    fclose(input);
    fclose(output);
}
