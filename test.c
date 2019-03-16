#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <string.h>
#include <stdbool.h>

bool inarray(double val, double arr[], int size)
{
    for (int i = 0; i < size; i++)
    {
        if (arr[i] == val)
        {
            printf("Val: %f\n", val);
            return true;
        }
    }
    return false;
}

static inline long part2(char *input, int input_size)
{
    double seen[10];
    double *seen_ind = seen;

    long sum = 0;
    char *token;
    char s[2] = "\n";
    char *ptr;
    long res;

    char copy[input_size];
    strcpy(copy, input);

    while (true)
    {

        token = strtok(copy, s);
        while (token != NULL)
        {
            *seen_ind = sum;
            res = strtol(token, &ptr, 10);
            printf("%ld\n", sum);
            sum += res;
            if (inarray(sum, seen, seen_ind - seen))
            {
                return sum;
                break;
            }
            token = strtok(NULL, s);
        }
        strcpy(copy, input);

    }

    return sum;
}

int main(void)
{
    char test[] = "+3\n +3 \n +4 \n -2 \n -4 \0";
    long output = part2(test, 10);
    fprintf(stdout, "%ld\n", output);
    return 0;
}