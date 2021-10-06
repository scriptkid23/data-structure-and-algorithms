package com.spirity.recursion;

public class Practice2 {
    public static int count = 0;
    public static int c(int k, int n){

        if(k == 1 && n == 2){
            count++;
        }
        if(k == n || k == 0) return  1;
        return c(k -1,n -1) + c(k,n -1);
    }
    public static void main(String[] args) {
        c(3,5);
        System.out.println(count);
    }
}
