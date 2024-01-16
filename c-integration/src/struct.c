#include "struct.h"
#include <stdio.h>

void seta (Struct* s, int32_t a) {
    s->a = a;
}

void setb (Struct* s, int16_t b) {
    s->b = b;
}

int32_t geta (Struct* s) {
    return s->a;
}

int16_t getb (Struct* s) {
    return s->b;
}

void print_Struct (Struct* s) {
    printf("a: %d\n", s->a);
    printf("b: %d\n", s->b);
}
