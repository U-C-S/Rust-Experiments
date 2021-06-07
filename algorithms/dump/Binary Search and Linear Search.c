#include<stdio.h>
#include<stdlib.h>
#include<time.h>

void delay(unsigned int msec)
{
    clock_t goal = msec+clock();
    while(goal > clock());
}
int binary(int item, int a[],int low,int high)
{
    int mid;
    mid=(low+high)/2;
    delay(1000);
    if(low>high)
        return 0;
    if(a[mid]==item)
        return mid+1;
    else if(a[mid]>item)
            return binary(item,a,low,mid-1);
    else
        return binary(item,a,mid+1,high);
}

int linear(int item, int n, int a[])
{
    int i;
    for(i=0;i<n;i++)
    {
        delay(100);
        if(a[i]==item)
            return i+1;
    }


    return 0;
}

void sort(int a[], int n)
{
    int i, j, temp;
    for(i=0;i<n-1;i++)
        for(j=i+1;j<n;j++)
        {
            if(a[i]>a[j])
            {
                temp=a[i];
                a[i]=a[j];
                a[j]=temp;
            }

        }
}
int main()
{

    int a[10000],n,pos,item, i, opt;
    clock_t start, end;
    double time;

    printf("Enter the array size:");
    scanf("%d",&n);

    printf("\nElements are:\n");
    for(i=0;i<n;i++)
    {

        a[i]=(int)rand()%1000;
        printf("%d\t", a[i]);

    }

    for(;;)
    {

        printf("\n1. Binary Search \n2. Linear Search \n3. Exit");
        printf("\nEnter your option:");
        scanf("%d",&opt);

        switch(opt)
        {

            case 1:
                    sort(a,n);
                    printf("\nSorted list:\n");
                    for(i=0;i<n;i++)
                        printf("%d\t",a[i]);
                    printf("\nEnter the key element:");
                    scanf("%d",&item);
                    start=clock();
                    pos=binary(item,a,0,n-1);
                    end=clock();
                    break;
            case 2:

                    printf ("Enter the Element to be searched:\n");
                    scanf ("%d",&item);
                    start=clock();
                    pos=linear(item,n,a);
                    end=clock();
                    break;
            default : exit(0);
        }
        if(pos==0)
        printf("Item not found\n");
        else
        printf("Item found at the position %d",pos);

        time=(end-start)/CLK_TCK;
        printf("\n Time taken = %f", time);
    }
    return 0;
}



//-------------------------------------

#include<stdio.h>
#include<stdlib.h>
//prims algo
int main()
{
    int n,u,v,
        min=999,mincost=0,cost[10][10],
        visited[10]={0},i,j,a,b,
        count=1;

    printf("Enter the number of vertices\n");
    scanf("%d",&n);

    printf("Enter the cost matrix.(Put 999 for Infinity)\n");
    for(i=1;i<=n;i++)
    {
        for(j=1;j<=n;j++)
        {
            scanf("%d",&cost[i][j]);
            if(cost[i][j]==0)
                cost[i][j]=999;
        }
    }
    visited[1]=1;

    printf("The edges of the Spanning tree are:\n");
    while(count<n)
    {
        min=999;
        for(i=1;i<=n;i++)
        {
            for(j=1;j<=n;j++)
            {
                if(cost[i][j] < min)
                {
                    if(visited[i]==0)
                        continue;
                    else
                    {
                        min=cost[i][j];
                        a=u=i;
                        b=v=j;
                    }
                }
            }
        }
         if(visited[u]==0 || visited[v]==0)
            {
                count++;
                printf("Edge(%d,%d) = %d\n",a,b,min);
                mincost +=min;
                visited[b]=1;
            }
            cost[a][b]=cost[b][a]=999;

    }
    printf("The minimum cost = %d\n",mincost);
    return 0;
}