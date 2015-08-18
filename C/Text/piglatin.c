#include "stdio.h"
#include "string.h"

int main() {
    printf("Enter string\n");
    char line[1024] = {'\0'};
    char *in = fgets(line, sizeof(line), stdin);
    // get rid of newline
    in[strlen(in) - 1] = '\0';

    int olen = strlen(in);
    in[olen] = '-';
    in[olen + 1] = in[0];
    in[olen + 2] = '\0';

    int x; for (x = 0; x < strlen(in) + 1; x++) {
        in[x] = in[x + 1];
    }
    strcat(in, "ay");

    printf("%s\n", in);
}
