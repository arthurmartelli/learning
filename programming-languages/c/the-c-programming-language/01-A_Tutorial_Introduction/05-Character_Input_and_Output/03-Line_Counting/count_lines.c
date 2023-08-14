#include <stdio.h>

/* count lines in input */

int main()
{
    int c, nl = 0, bl = 0, tb = 0;

    while ((c = getchar()) != EOF)
    {
        if (c == '\n')
        {
            ++nl;
        };
    }
    // printf("nl = %d bl = %d tb = %d\n", nl, bl, tb);
    printf("nl: %d", nl);
}