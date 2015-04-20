#include <stdio.h>
#include <string.h>

/*
 * I am aware of the potential problems here, just a proof of concept
 */
int main(int argc, char *argv[]) {
  int length = 0;
  
  int i;
  for (i = 1; i < argc; i++) {
    length += strlen(argv[i]);
  }
  char string[length];
  for (i = 1; i < argc; i++) {
    strcat(string, argv[i]);
    if (i + 1 < argc) {
      strcat(string, " ");
      length ++;
    }
  }

  printf("Reversing: %s\n", string);
  char temp[length];
  int x;
  for (x = 0; x < length; x++) {
    temp[x] = string[length - x - 1];
  }
  printf("Reversed: %s\n", temp);
  return 0;
}
