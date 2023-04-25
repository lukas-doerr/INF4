#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define N 6
// const int n = 4;
int pool[N];
int stop[N];

void print(int* A)
{
    for (int i = 0; i<N; i++)
    {
        printf("%d", A[i]);
    }
    printf("\n");
}

void swap(int* A)
{
    int tmp;
    tmp = A[N-1];
    A[N-1] = A[N-2];
    A[N-2] = tmp;
}

void check(int* A, int* mask, int rol)
{
    int *p = &A[0];
    for (int i = 0; i < N; i++) {
        mask[i] = 0;
    }
    int k = 1;
    while(p!=&A[N-2-rol])
    {
        for (int i = 0; i < N; i++) {
            if(*p == pool[i]) mask[i] = k;
        }
        p++;
    }
    //print(mask);
}

int compare(int* A)
{
    int is_same = 1;
    for (int i = 0; i < N; i++) {
       if(A[i] != stop[i]) is_same = 0;  
    }
    return is_same;
}

int increment(int* p, int* A)
{
    int* lp = &A[0];
    while(lp!=p){
        if(((*p)+1 == *lp) && ((*p)+1 != N))
        {
            (*p)++;
            increment(p, A);
        
        }
        lp++;
    }
}

int move(int* p, int* A, int* rol)
{
    int inc = N;

    
    int* lp = &A[0];
    int count = 0;
    while(lp!=p){
        if (*lp > *p)
        {
            count++;
        }
        lp++;
    }
    
    if((*p >= N-count))
    {
        p--;
        (*rol)++;

        if(*p != N)
        {
            (*p)++;

            int* lp = &A[0];
            int i;
            do{
                i=0;
                while(lp!=p){

                    if((*p == *lp) && (*p!=N))
                    {
                        (*p)++;
                        i=1;
                        printf("hh");
                    }
                    lp++;
                }
            }while(i);


        }
        else
        {
            move(p, A, rol);
        } 
        printf("H");
        p++;
    }
}

void perm(int* A)
{
    static int cnt = 1;
    
    int mask[N];
   
    

    
    printf("%d:", cnt);
    print(A);

    cnt++;
    //if( cnt == 130 ) return;
    int *p = &A[N-3];
    swap(A);
    printf("%d:", cnt);
    print(A);
    if(compare(A)) return;
    cnt++;

    int rol = 0;

    move(p, A, &rol);

    //printf("\n This is rol: %d\n", rol);

    increment(p, A);

    int inc = 1;

    for(int i=0; i < N-2; i++)
    {
        if((A[i] == N) && (*p == N-1))
        {
            inc = 0;
        }
    }

    if(inc) (*p)++;

    check(A, mask, rol);

    int k = 2+rol;
    for (int i = 0; i < N; i++) {
        if(!mask[i])
        {
            A[N-k] = pool[i];
            k--;
        }
    }


    perm(A);

}

int main()
{
    int A[N];
    for (int i = 0; i < N; i++) {
        pool[i] = i+1;
        stop[i] = N-i;
        A[i] = i+1;
    }
    perm(A);
}