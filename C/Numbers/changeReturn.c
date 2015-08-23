#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  double money = atof(argv[1]);
  double amounts[10] = {50, 20, 10, 5, 2, 1, 0.25, 0.10, 0.05};

  int x;
  for (x = 0; x < 9; x++) {
    int a = money / amounts[x] + 0.01; // yes, this has some edge cases but this is not serious
    money -= a * amounts[x];
    printf("%.2f's\t: %d\n", amounts[x], a);
  }
}
