#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>


void printBits(size_t const size, void const * const ptr)
{
    unsigned char *b = (unsigned char*) ptr;
    unsigned char byte;
    int i, j;
    
    for (i = size-1; i >= 0; i--) {
        for (j = 7; j >= 0; j--) {
            byte = (b[i] >> j) & 1;
            printf("%u", byte);
        }
    }
    puts("");
}


int addBit(int b)
{
   
}

int main(int argc, char* argv[])
{
    uint8_t bit = 0;

    for(int i=0; i<20; i++)
    {
        uint8_t x = ~bit;
        printBits(1, &x);
    }
    return 0;
}

