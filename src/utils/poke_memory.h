#ifndef POKE_MEMORY_H
#define POKE_MEMORY_H 
    #include "../utils/common.h"

    #define CAPACITY_MAX_SIZE 8

    #define GROW_ARRAY(type, pointer, old_count, new_count) \
        (type*) reallocate(pointer, sizeof(type) * old_count, sizeof(type) * new_count)    
        
        
    int grow_capacity(int capacity);
    void* reallocate(void* pointer, size_t old_size, size_t new_size);
#endif