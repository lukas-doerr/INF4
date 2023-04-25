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


void findSuf(int* A) {
    static int cnt = 0;
    int max = 0;
    int suf = N-1;


    for(int i=N-1; i>=1; i--)
    {
        int max=0;
        for(int j=1; j<i; j++)
        {
            if(A[j] > max) max = A[j];
        }

        if(A[i] <= max)
        {
            A[i]++;
            cnt++;
            printf("%d:\t", cnt);
            print(A);
            break;
        }
        else{
            A[i] = 0;
        } 
    }


    if(A[N-1] == N-1) return;
    findSuf(A);
}

int main() {
    int A[N];
    int cnt = 0;
    fill(A, N, 0);
    
    printf("%d:\t", cnt);
    print(A);

    findSuf(A);


    return;    
}