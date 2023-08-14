#include <stdio.h>

/* print Fahrenheit-Celsius table
for fahr = 0, 200, ..., 300 */

int title(char *string)
{
    printf("------------------------------\n");
    printf("%s\n", string);
    printf("------------------------------\n");
    return 0;
};

int main()
{
    float fahr, celsius;
    int f_lower, f_upper, f_step;
    int c_lower, c_upper, c_step;

    f_lower = 0;   // lower limit of temperature table
    f_upper = 300; // upper limit
    f_step = 20;   // step size

    title("Fahrenheit to Celsius Table");

    fahr = f_lower;
    while (fahr <= f_upper)
    {
        celsius = (5.0 / 9.0) * (fahr - 32.0);
        printf("%3.0f %6.1f\n", fahr, celsius);
        fahr = fahr + f_step;
    };

    title("Celsius to Fahrenheit Table");

    c_lower = -20; // lower limit of temperature table
    c_upper = 100; // upper limit
    c_step = 10;   // step size

    celsius = c_lower;
    while (celsius <= c_upper)
    {
        fahr = (celsius * 9.0 / 5.0) + 32.0;
        printf("%3.0f %6.1f\n", celsius, fahr);
        celsius = celsius + c_step;
    };

    return 0;
};
