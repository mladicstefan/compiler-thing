#include <stdio.h>

extern int add(int a, int b);

extern void vuln(char *p);

int main()
{
    int x = add(3, 7);
    printf("%d\n", x);
    char input[64];
    fgets(input, sizeof(input), stdin);
    vuln(input);
    printf("Returned normally\n");
    return x;
}
