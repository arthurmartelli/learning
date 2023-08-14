#include <stdio.h>

#define LOWER 0   // lower limit of table
#define UPPER 300 // upper limit
#define STEP 20   // step size

/* print Fahrenheit-Celsius table */

float celsius(int fahr)
{
    return (5.0 / 9.0) * (fahr - 32);
}

int main(int argc, char const *argv[])
{
    int fahr, degree;

    for (fahr = LOWER; fahr <= UPPER; fahr = fahr + STEP)
    {
        printf("%3d %6.1f\n", fahr, celsius(fahr));
    }
}
