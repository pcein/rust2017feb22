#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdio.h>

int main() 
{
    int fd, n;
    char buf[1024];

    // "open" may fail ...    
    fd = open("data.txt", O_RDONLY);
 
    // if "open" fails, we are passing a
    // meaningless value to "read", which also
    // will fail.
    // There is nothing the type system can do to
    // to prevent this.

    n = read(fd, buf, sizeof(buf));
       
    printf("read %d bytes \n", n);

}

