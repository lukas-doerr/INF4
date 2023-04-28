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

int count(int* Zahl, int* Base)
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

    if(suf<0) return -1;


    int i = suf; 
    if(i!=N-1){
        do
        {
            Zahl[i] = 0;
            i++;
        }while(Zahl[i]<=Zahl[N-1]);
    }

    Zahl[suf]++;
    //print(Zahl);
    //count(Zahl, Base);
    return suf;
}

void turn(int* A)
{
    int tmp=A[0];

    for(int i=1; i<N; i++) 
    {
        A[i-1] = A[i];
    }
    A[N-1] = tmp;
}

void copy(int* A, int* B)
{
    for(int i=0; i<N; i++)
    {
        B[i] = A[i];
    }
}

int greater(int* A, int* B){
    int great = 0;
    for(int i=0; i<N; i++) {
        if(A[i] != B[i]) great=1;
    }

   // printf("--> %d", great);
    return great;
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
    return res;    
}

int compare(int* Zahl, int* Base)
{
    int cnt = 0, res = 0;
    int* cpy[N], cpyB[N];
    int val = mixedToDec(Zahl, Base);
    copy(Zahl, cpy);
    copy(Base, cpyB);

    for(int i=0; i<N-1; i++){
        turn(cpy);
        int dec = mixedToDec(cpy, cpyB);
        //res = greater(Zahl, cpy);
       if(dec >= val) cnt++;
    }
    //printf("\t%d\n", cnt);
    return cnt;
}

int main() 
{
    int Zahl[N], Base[N];
    int res = 0, suf = 0, cnt = 0;

    fill(Zahl, N, 0);
    fill(Base, N, 2);
    
   // count(Zahl, Base);
   // compare(Zahl);
   // return;

    do{
        suf = count(Zahl, Base);
        if(compare(Zahl, Base) == 3 && suf>=0) {
            cnt++;
            printf("%d Found: ", cnt);
            print(Zahl);
            res++;
        }
    }while(suf >= 0);
    //count(Zahl, Base);
    
}
