#ifndef CHUNK_H
#define CHUNK_H
    #include "../utils/common.h"
    #include <memory.h>

    #define GROW_CAPACITY(capacity) ""

    typedef enum {
        OP_RETURN,
    } op_code_t;

    // count <= capacity
    typedef struct {
        int count;
        int capacity;

        uint8_t* code;
    } chunk_t;

    void init_chunk(chunk_t* chunk);
    void write_chunk(chunk_t* chunk, uint8_t byte);
#endif