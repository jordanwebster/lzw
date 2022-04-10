#include <stdio.h>
#include <string.h>

#include "buffer.h"

void buffer_reset(buffer_t *buffer)
{
    memset(&buffer->buf, 0, sizeof(char) * 3);
    buffer->is_first_code = true;
}

void buffer_init(buffer_t *buffer, FILE *output)
{
    buffer->output = output;
    buffer_reset(buffer);
}

void buffer_write(buffer_t *buffer, int code)
{
    if (buffer->is_first_code) {
        buffer->buf[0] = code & 0x0FF0;
        buffer->buf[1] = code & 0x000F;
        buffer->is_first_code = false;
    } else {
        buffer->buf[1] |= code & 0x0F00;
        buffer->buf[2] = code & 0x00FF;
        fwrite(&buffer->buf, sizeof(char), 3, buffer->output);
        buffer_reset(buffer);
    }
}

void buffer_flush(buffer_t *buffer)
{
    if (!buffer->is_first_code)
    {
        fwrite(&buffer->buf, sizeof(char), 2, buffer->output);
    }
}
