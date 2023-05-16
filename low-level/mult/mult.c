#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <assert.h>

ulong mul(ulong x, ulong y, ulong m )
{
	ulong tmp=0;
	while(y!=0)
	{	
		if(y & 1){
			tmp += x;
			if ( tmp >= m )  tmp -= m;
		}
		x = x << 1;
		if ( x >= m )  x -= m;
		y = y >> 1;
	}
	return tmp;
}

int main() //(int arg, char* argc)
{

	ulong x=1111111111, y=1111111111, m = 1000000000000;
	assert( x < m );
	assert( y < m );
	ulong p = mul(x,y,m);
	printf("%ld\n", p);
	printf("%ld\n", (x*y) % m);

}
