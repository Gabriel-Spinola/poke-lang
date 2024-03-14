#include "chunk.h"
#include "../utils/poke_memory.h"
#include <memory.h>

void init_chunk(chunk_t* chunk) {
    chunk->count = 0;
    chunk->capacity = 0;

    chunk->code = NULL;
}

void write_chunk(chunk_t* chunk, uint8_t byte) {
    if (chunk->capacity < chunk->count + 1) {
        int old_capacity = chunk->capacity;
        
        chunk->capacity = grow_capacity(old_capacity);
        chunk->code = GROW_ARRAY(uint8_t, chunk->code, old_capacity, chunk->capacity);
    }

    chunk->code[chunk->count] = byte;
    chunk->count++;
}
