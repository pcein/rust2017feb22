#include <stdlib.h>

void foo() 
{
    char *p;

    // space allocated on the heap
    
    p = malloc(100 * sizeof(char));

    // the allocated memory is NOT automatically
    // free'd. You must call the "free" function
}

int main()
{
    foo();
    return 0;
}

