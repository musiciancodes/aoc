#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <string.h>
#include <stdbool.h>

static inline void failure(const char *fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    vfprintf(stderr, fmt, args);
    va_end(args);

    fputc('\n', stderr);
    fflush(stderr);
    exit(1);
}

/**
 * Your solution goes here. As input, it is provided the complete input to the
 * problem (read from stdin) as a character array, and the number of bytes in
 * the input. It should return an integer, which will be printed to stdout. It
 * can also call `failure` if an invariant fails and you need to exit early.
 *
 * input is guarenteed to be null terminated, so C string functions (like strlen
 * and strtok) can safely work on it. input_size is the number of characters read
 * from stdin, excluding this null terminator.
 */
static inline long part1(char *input, size_t input_size)
{
    return 0;
}


static inline long part2(char *input, size_t input_size)
{
 return 0;
}

/**
 * Main function. Reads all of stdin into a buffer; calls solve(); prints the solution
 */
int main()
{
    // Start with 64k, which should cover most inputs without reallocating
    size_t allocated = 65536;
    char *buffer = malloc(allocated);
    size_t buffer_size = 0;

    // Read stdin, reallocating as necessary, until we reach EOF
    while (!feof(stdin))
    {
        // Note that we use (buffer_size + 1) throughout this function because we
        // want to make sure to leave some extra space to append a null byte after
        // reading in stdin.

        // If necessary, reallocate the buffer. See https://en.wikipedia.org/wiki/Dynamic_array#Growth_factor
        // for details of why we multiply by 3/2.
        if ((buffer_size + 1) >= allocated)
        {
            allocated = (allocated * 3) / 2;
            buffer = realloc(buffer, allocated);
        }

        // Read some more input
        const size_t count = fread(buffer + buffer_size, 1, (allocated - (buffer_size + 1)), stdin);
        buffer_size += count;

        // Check for errors
        if (ferror(stdin))
        {
            perror("Error reading from stdin");
            return 1;
        }
    }
    // Append a null byte to make sure that c string functions are safe.
    buffer[buffer_size] = 0;

    char *buffer2 = malloc(allocated);
    strcpy(buffer2, buffer);

    printf("allocated: %ld\n", allocated);
    printf("buffer size: %ld\n", buffer_size);
    // Solve the puzzle
    const long solution1 = part1(buffer, buffer_size);
    const long solution2 = part2(buffer2, buffer_size);

    // Print the solution to the puzzle
    // fprintf(stdout, "%ld\n", solution1);
    // fflush(stdout);
    // fprintf(stdout, "%ld\n", solution2);
    // fflush(stdout);

    // Check for errors
    if (ferror(stdout))
    {
        perror("Error writing solution to stdout");
        return 1;
    }

    // Free the buffer. There's no reason to do this here because we're about to
    // exit the program, but we'd prefer the code in here to exhibit good C style.
    free(buffer);
}
