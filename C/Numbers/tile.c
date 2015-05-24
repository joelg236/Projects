#include <stdio.h>
#include <stdlib.h>

char *line() {
  char *line = NULL;        
  size_t size;
  getline(&line, &size, stdin);
  return line;
}

int main() {
  printf("Enter width\n");
  int width = atoi(line());
  printf("Enter height\n");
  int height = atoi(line());
  printf("Enter cost\n");
  double cost = atof(line());

  printf("Total cost = %f\n", (width * height) * cost);
}
