#include <stdlib.h>
#include <stdio.h>

#define N 30


int Base[N];
int Zahl[N];
int R[N];

void fill()
{
    for(int i=0; i < N; i++)
    {
        Zahl[i] = 0;
        //stop[i] = 0;
        Base[i] = 2;
        R[i] = 1;
    }
}



void print(int* A)
{
    for (int i = 0; i<N; i++)
    {
        printf("%d", A[i]);
    }
}

void next2()
{
    static int m = 1;
    int i = N, br = 0;
    int suf = N-1;



    while((suf >= 0) && !br)
    {   
        
        if((Zahl[suf] == Base[suf]-1) && (R[suf] == 1))
        {
            R[suf] = 0;   
            suf--;  
        }
        else if((Zahl[suf] == 0) && (R[suf] == 0))
        {
            R[suf] = 1;
            suf--;
        }
        else{
            br = 1;
        }

    }

    if(suf < 0)
    {
        //printf("%d", m);
        return;
    } 

    m++;
    if((R[suf] == 1)) Zahl[suf]++;
    else Zahl[suf]--;
    next2();
}



void next()
{   //static int m = 1;
    int suf = N, br = 0;
    int* p = &Zahl[N-1];
    int* bp = &Base[N-1];
    int* rp = &R[N-1];
    
    

    /*printf("%d: ", m);
    print(Zahl);
    printf("\t");
    print(R);
    printf("\n");*/


    
    
    while((&Zahl[0] <= p) && !br)
    {   
        
        if((*p == (*bp)-1) && (*rp == 1))
        {
            suf--;
            *rp = 0;    
            p--;
            rp--; 
            bp--;  
        }
        else if((*p == 0) && (*rp == 0))
        {
            suf--;
            *rp = 1;
            p--;
            rp--;  
            bp--; 
        }
        else{
            br = 1;
        }
    }

    if(suf == 0)
    {
        //printf("%d", m);
        return;
    } 


    if((suf > 0) && ((*rp) == 1)) Zahl[suf-1]++;
    if((suf > 0) && ((*rp) == 0)) Zahl[suf-1]--;
    
    //m++;
    next();
}


int main()
{
    fill();
    next2();
}