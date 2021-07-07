package com.spirity.Graph;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Scanner;

public class practice2 {
    public static void readFile(String filename){
        try{
            File myObj = new File(filename);
            Scanner myReader = new Scanner(myObj);
            int N=myReader.nextInt();
            int map[][] = new int[N][N];
            for(int i = 0; i < N; i++){
                for(int j = 0; j < N; j++){
                    map[i][j] = myReader.nextInt();
                }
            }
            int K=myReader.nextInt();
            int F[] = new int[N];
            for(int i = 0; i < N; i++){
                F[i] = Integer.MAX_VALUE;
            }
            Queue<Integer> Q = new LinkedList<>();

            int d[] = new int[N];
            int h[] = new int[N];

            for(int i= 0; i < K; i++){
                int index=  myReader.nextInt();
                System.out.println("FO Trace: "+index);

                F[index - 1] = 0;
                d[index - 1] = 1;
                d[index - 1] = 0;

                Q.add(index - 1);

                while (!Q.isEmpty()){
                    int u = Q.poll();

                    for(int v = 0; v < N; v++){
                        if(map[u][v] == 1 && d[v] == 0){
                            Q.add(v);
                            h[v] = h[u] + 1;
                            d[v] = 1;
                            if(F[v] > h[v]) F[v] = h[v];
                        }

                    }
                }
                for(int j = 0; j <N; j++) d[j] = 0;
                for(int k = 0; k <N; k++) h[k] = 0;
            }
            System.out.println();
            for(int i = 0; i < N;i++){
                if(F[i] == Integer.MAX_VALUE) F[i]= -1;
                System.out.print(F[i]+",");
            }






        }catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }
    }
    public static void main(String args[]){
        readFile("C:\\Users\\hoando\\Downloads\\dataset_.txt");
    }
}
