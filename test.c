#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <string.h>
#include <stdbool.h>

typedef struct DOUBLE_ARR {
    int count;
    long* arr;
} DOUBLE_ARR;

bool inarray(long val, long arr[], int size)
{
    for (int i = 0; i < size; i++)
    {
        if (arr[i] == val)
        {
            printf("Val: %ld\n", val);
            return true;
        }
    }
    return false;
}
// Converts the string into an array of signed ints
struct DOUBLE_ARR numArray(char *input, int input_size)
{
    // each line is at least three characters: sign, int, newline
    long* arr = malloc (input_size * sizeof(long));
    long* arr_ind = arr;
    int count = 0;

    char* token;
    char s[2] = "\n";
    char* ptr;
    long res;
    token = strtok(input, s);
    while (token != NULL) {
        res = strtol(token, &ptr, 10);
        *arr_ind = res;
        token = strtok(NULL, s);
        arr_ind++;
        count ++;
    }
        
    DOUBLE_ARR returnArr = {count, arr};
    return returnArr;
}

static inline long part2(char *input, int input_size)
{
  	// Start with 64k, which should cover most inputs without reallocating
	size_t allocated = 65536;
	long* seen_buff = malloc(allocated);
    long* seen_ind = seen_buff;
	size_t seen_size = 0;

    DOUBLE_ARR array = numArray(input, input_size); 
    long* arr = array.arr;
    int item_count = array.count;

    
    bool already_seen = false;

	// Read stdin, reallocating as necessary, until we reach EOF
    int item_ind = 0;
    long sum = 0;
	while(!already_seen) {
        *seen_ind = sum;
        seen_ind++;
        
        // make wrap around the index
        if (item_ind == item_count){
            item_ind = 0;
        }
        sum += arr[item_ind];
        item_ind++;
        already_seen = inarray(sum, seen_buff, seen_size);

		// If necessary, reallocate the buffer. See https://en.wikipedia.org/wiki/Dynamic_array#Growth_factor
		// for details of why we multiply by 3/2.
		if((seen_buff + 1) >= allocated) {
			allocated = (allocated * 3) / 2;
			seen_buff = realloc(seen_buff, allocated);
		}

		seen_size += 1;
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