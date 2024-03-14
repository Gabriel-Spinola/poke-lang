#include "poke_memory.h"

int grow_capacity(int capacity) {
    if (capacity < CAPACITY_MAX_SIZE) {
        return CAPACITY_MAX_SIZE;
    }

    return capacity * 2;
}