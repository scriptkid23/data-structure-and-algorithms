package com.spirity.recursion;


public class Practice1 {
    public static int f(int m, int n){
        if(m == 0) return n + 1;
        if(m > 0 && n == 0) return f(m-1,1);
        return f(m-1, f(m,n-1));
    }
    public static void main(String args[]){
        System.out.println(f(2,2));
    }
}
