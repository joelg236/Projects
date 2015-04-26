#include <stdio.h>
#include <string.h>

int main(int argc, char *argv[]) {
  int n = atoi(argv[1]);
  int factors[n];
  memset(factors, 0, sizeof factors);

  int x, i = 0;
  for (x = 2; x < n+1; x++) {
    if (n % x == 0) {
      factors[i++] = x;
      n = n / x;
      x = 1;
    }
  }
  
  for (x = 0; x < sizeof(factors) / sizeof(factors[0]); x++) {
    if (factors[x] != 0) {
      printf("%d, ", factors[x]);
    }
  }
  printf("\n");
}
