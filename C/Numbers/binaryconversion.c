#include <stdio.h>
#include <math.h>

int main() {
    char t;
    printf("Integer (i) or binary (b) input? ");
    scanf("%c", &t);

    printf("Enter number: ");
    long long unsigned int i;
    scanf("%llu", &i);

    if (t == 'i') {
        int start = 0;
        for (int x = 31; x >= 0; x--) {
            if (i >= (unsigned int)pow(2, x)) {
                start = 1;
                printf("1");
                i -= (int)(pow(2, x));
            } else if (start) {
                printf("0");
            }
        }
        printf("\n");
    } else if (t == 'b') {
        if (i > 1111111111111111111) return -3; // meh, just can't handle it. don't care enough.
        unsigned long long int decimal=0, index=0;
        while (i != 0) {
            decimal += (i % 10) * (int) pow(2, index++);
            i/=10;
        }
        printf("%u\n", decimal);
    }
}
