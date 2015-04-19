#include <stdio.h>
#include <string.h>
#include <math.h>

int main(int argc, char *argv[]) {
  int digits = atoi(argv[1]);
  int coef = 1;
  int x;
  for (x = 0; x < digits; x++) {
    coef *= 10;
  }
  float result = roundf(M_PI * coef) / coef;
  printf("PI to %d digits is %.*f\n", digits, digits, result);
}
