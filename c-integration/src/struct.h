#ifndef STRUCT_H
#define STRUCT_H

#include <stdint.h>

typedef struct Struct {
    int32_t a;
    int16_t b;
} Struct;

void seta (Struct*, int32_t);
void setb (Struct*, int16_t);
int32_t geta (Struct*);
int16_t getb (Struct*);

#endif
