#include <stdio.h>
#include <string.h>

int nextPrime(int old) {
  for (old = old+1;;old++) {
    int d;
    int found = 1;
    for (d = 2; d < old; d++) {
      if (old % d == 0) {
        found = 0;
        break;
      }
    }

    if (found) return old;
  }
}

int main() {
  printf("Space for next prime, q or C-c to quit.");
  int prime = 1;
  for (;;) {
    char l = getchar();
    if (l == 'q') {
      break;
    } else {
      prime = nextPrime(prime);
      printf("Next prime is %d", prime);
    }
  }
  return 0;
}
