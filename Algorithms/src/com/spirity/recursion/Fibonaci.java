package com.spirity.recursion;

public class Fibonaci {
    static int cache[] = new int[200];
    static public int f_cache(int k){

        if(k == 0){
            return 0;
        }
        if (k == 1){
            return 1;
        }
        if(cache[k] != 0){
            return cache[k];
        }
        else{
            cache[k] = f_cache(k-1) + f_cache(k-2);
            return f_cache(k-1) + f_cache(k-2);
        }

//        System.out.println("F["+k+"]:"+(f(k-1) + f(k-2)));
//        if(cache[k] != 0 || cache[k] != 1) return cache[k];

    }
    static public int f_normal(int k){

        if(k == 0){
            return 0;
        }
        if (k == 1){
            return 1;
        }


        return f_normal(k-1) + f_normal(k-2);


//        System.out.println("F["+k+"]:"+(f(k-1) + f(k-2)));
//        if(cache[k] != 0 || cache[k] != 1) return cache[k];

    }
    public static void main(String args[]){


//        long startTime2 = System.nanoTime();
//        f_normal(140);
//        long endTime2   = System.nanoTime();
//        long totalTime2 = endTime2 - startTime2;
//        System.out.println("Execution time in nanoseconds  : " + totalTime2);
//        System.out.println("Execution time in milliseconds : " + totalTime2 / 1000000);

        long startTime1 = System.nanoTime();
        int value = f_cache(30);
        System.out.println(value);
        long endTime1   = System.nanoTime();
        long totalTime1 = endTime1 - startTime1;
        System.out.println("Execution time in nanoseconds  : " + totalTime1);
        System.out.println("Execution time in milliseconds : " + totalTime1 / 1000000);
    }
}
