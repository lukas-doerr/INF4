#include <stdlib.h>
#include <stdio.h>

#define N 3

int Base[N];
int Zahl[N];

void fill()
{
    for(int i=0; i < N; i++)
    {
        Zahl[i] = 0;
        Base[i] = i+2;
    }
}

void print(int* A)
{
    for (int i = 0; i<N; i++)
    {
        printf("%d", A[i]);
    }
    printf("\n");
}


void next()
{   static int m = 0;
    int suf = N, br = 0;
    int* p = &Zahl[N-1];
    int* bp = &Base[N-1];

    
    while((&Zahl[0] <= p) && !br)
    {
        if(*p == (*bp)-1)
        {
            suf--;
            p--;
            bp--;
        }
        else{
            br = 1;
        }
        
    }

    if(suf==0) return;
    
    int* lp;
    if(suf!=N){
        lp = &Zahl[suf];
        do
        {
            *lp = 0;
            lp++;
        }while(lp<=&Zahl[N-1]);
    }
    



    if(suf) Zahl[suf-1]++;
    print(Zahl);
    m++;
    if(m==100) return;
    next();

}

int main()
{
    fill();
    next();
}