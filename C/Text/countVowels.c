#include <stdio.h>
#include <string.h>

int main() {
  printf("Enter string\n");
  char line[1024] = {'\0'};
  char *in = fgets(line, sizeof(line), stdin);
  
  char vowels[5] = {'a', 'e', 'i', 'o', 'u'};
  int x, sum = 0;
  for (x = 0; x < strlen(in); x++) {
    int i;
    for (i = 0; i < strlen(vowels); i++) {
      if (vowels[i] == in[x]) {
        sum++;
      }
    }
  }

  printf("Your string had %d vowels\n", sum);
}
