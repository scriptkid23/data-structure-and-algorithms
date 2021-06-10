package com.spirity.Dynamic;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class practice3 {
    public static int max(int a, int b, int c){
        int max = a;
        if( max < b) max = b;
        if( max < c) max = c;
        return max;
    }
    public static void readFile(String filename){
        try{
            File myObj = new File(filename);
            Scanner myReader = new Scanner(myObj);
            int N=myReader.nextInt();
            int M=myReader.nextInt();
            int map[][] = new int[N][M];
            int f[][] = new int[N+1][M+1];

            System.out.println("N:"+N);
            System.out.println("M:"+M);

            for(int i = 0; i < N;i++){
                for(int j = 0; j < M; j++){
                    map[i][j]  = myReader.nextInt();
                }
            }
            for(int i = 1; i <= N;i++){
                f[i][0] = Integer.MIN_VALUE;
            }
            for(int j = 1; j <= M;j++){
                f[0][j] = Integer.MIN_VALUE;
            }
            f[0][0] = 0;

            for(int i = 1; i <= N;i++){
                for(int j = 1; j <=M;j++){
                    if(map[i-1][j-1] == - 1){
                        f[i][j] = Integer.MIN_VALUE;
                    }
                    else{
                        f[i][j] = max(f[i-1][j],f[i][j-1],f[i-1][j-1])+map[i-1][j-1];
                    }
                }
            }
            System.out.println(f[N][M]);
        }
        catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }

    }
    public static void main(String args[]){

        readFile("C:\\Users\\hoando\\Downloads\\dataset.txt");

    }
}
