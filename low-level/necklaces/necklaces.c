#include <stdlib.h>
#include <stdio.h>

#ifndef N
#define N 4
#endif

void print(int* A)
{
    for (int i = 0; i<N; i++)
    {
        printf("%d", A[i]);
    }
    printf("\n");
}

void fill(int* A, int n, int val)
{
    for (int i = 0; i < n; i++) {
        A[i] = val;
    }
}

void count(int* Zahl, int* Base)
{
    int suf = N-1, br=0;
    static int cnt = 0;
    while((suf >= 0) && !br)
    {     
        if((Zahl[suf] == Base[suf]-1))
        {
            suf--;  
        }
        else{
            br = 1;
        }
    }

    if(suf<0) return;


    int i = suf; 
    if(i!=N-1){
        do
        {
            Zahl[i] = 0;
            i++;
        }while(Zahl[i]<=Zahl[N-1]);
    }

    Zahl[suf]++;
    print(Zahl);
}

void findNeck(int* A, int* B)
{



}


int decToMixed(int dec, int* Base)
{
    int mixed[N];
    for(int i=N-1; i>=0; i--)
    {
        mixed[i] = dec % Base[i];
        dec = dec / Base[i];
    }

    return mixed;  
}

int mixedToDec(int* Zahl, int* Base)
{
    int res = 0;
    for(int i=0; i<N; i++)
    {
        res = res * Base[i] + Zahl[i];
    }
    printf("%d", res);
    return res;    
}

void compare(int* Zahl, int* Min, int* Base)
{
    int dec = mixedToDec(Zahl, Base);
    int found = 0;
    
    for(int i=0; i<10; i++)
    {
        if(Min[i] < 0) return;

        if(dec==Min[i])
        {
            found = 1;
            break;
        }
    }

    if(found) return;

}

int main() 
{
    int Zahl[N], Base[N];
    int min[100];
    fill(Zahl, N, 1);
    fill(Base, N, 16);
    fill(min, 100, -1);
    print(Zahl);


    //compare(Zahl, min);
    //count(Zahl, Base);
    
}