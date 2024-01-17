#include "struct_t.hpp"
//#include <cstdint>

struct_t::struct_t () {
    a = 2;
    b = 312;
    c = 4e5;
    d = 6e10;
    e = 3.14e-6;
    f = 5.27e-12;
    g = 'z';
}

/* ret60 must have the const declaration on the end of the function prototype in order to
 * avoid a "mutable double borrow" error thrown by the Rust compiler because of how the two
 * member functions are used here.
 */
uint32_t struct_t::ret60 () const {
    return 60;
}

uint16_t struct_t::double_this (uint16_t x) {
    return x * 2;
}
