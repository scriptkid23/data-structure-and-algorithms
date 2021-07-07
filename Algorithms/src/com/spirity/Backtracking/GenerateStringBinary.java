package com.spirity.Backtracking;

public class GenerateStringBinary {
    static int n = 4;
    static int binary[] = new int[4];

    public static void Try(int k){
        if(k == n){
            for(int i = 0; i < binary.length; i++){
                System.out.print(binary[i]);
            }
            System.out.println();
            return;
        }
        for(int i = 0; i < 2; i++){
            System.out.println("binary["+k+"]:"+i);
            binary[k] = i;
            Try(k+1);
        }
    }
    public static void main(String[] args){
        Try(0);
    }
}
