#ifndef STRUCT_T_HPP
#define STRUCT_T_HPP

#include <cstdint>

class struct_t {
public:
    uint8_t a;
    uint16_t b;
    uint32_t c;
    uint64_t d;
    float e;
    double f;
    char g;

    struct_t();

    /* ret60 must have the const declaration on the end of the function prototype in order to
     * avoid a "mutable double borrow" error thrown by the Rust compiler because of how the two
     * member functions are used here.
     */
    uint32_t ret60 () const;
    uint16_t double_this (uint16_t);
};

#endif
