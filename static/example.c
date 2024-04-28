
#include <stdint.h>
#include <stdio.h>

struct S {
  int i;
  double j;
  char k;
};

int main() {
  struct S test = {2, 3.14, 'n'};

  for (int i = 0; i < sizeof(typeof(test)); i++) {
    printf("%02x", (unsigned int)((uint8_t *)&test)[i]);
  }
}
// 02000000 fa000000 1f85eb51b81e0940 6e 23805c1c020000
