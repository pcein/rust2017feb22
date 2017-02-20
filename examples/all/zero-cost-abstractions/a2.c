#include<stdio.h>
main()
{
    const unsigned long n = 100000000;
    
    unsigned long i, sum = 0;

    for(i = 0; i < n; i++) {
        sum += i;
    }

    printf("%lu\n", sum);
}
