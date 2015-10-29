#include <stdlib.h>

size_t shift_left(void* f, unsigned short pass) {
  return (size_t)((unsigned char)(*((unsigned int*)f) >> pass * 8));
}

size_t shift_right(void* f, unsigned short pass) {
  return (size_t)((unsigned char)(*((unsigned int*)f) << pass * 8));
}
