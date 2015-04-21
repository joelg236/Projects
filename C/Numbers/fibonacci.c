#include <stdio.h>

/*
 * Not zero-based argument!
 */
int main(int argc, char *argv[]) {
  int N = atoi(argv[1]);
  int fib[N];
  int x;
  for (x = 0; x < N; x++) {
    if (x <= 1) {
      fib[x] = x;
    } else { 
      fib[x] = fib[x-1] + fib[x-2];
    }
  }
  printf("%d\n", fib[N-1]);
  return 0;
}
