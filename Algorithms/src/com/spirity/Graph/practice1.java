package com.spirity.Graph;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Scanner;

public class practice1 {
    public static void readFile(String filename){
        try{
            File myObj = new File(filename);
            Scanner myReader = new Scanner(myObj);
            int N=myReader.nextInt();
            int K=myReader.nextInt();
            int A[][] = new int[N][N];
            for(int i = 0; i < N; i++){
                for(int j = 0; j < N; j++){
                    A[i][j] = myReader.nextInt();
                }
            }
            ArrayList<ArrayList<Integer>> components = new ArrayList<>();
            int d[] = new int[N];
            for(int i = 0; i < N; i++){
                if(d[i] == 1) continue;
                Queue<Integer> Q = new LinkedList<>();
                Q.add(i);
                d[i] = 1;
                ArrayList<Integer> newComponent = new ArrayList<>();
                while (!Q.isEmpty()){
                    int u = Q.poll();
    //                d[u] = 1; // danh dau da duyet
                    for(int v = 0; v < N; v++){
                        if(d[v] == 0 && A[u][v] == 1){
                            d[v] = 1;
                            Q.add(v);
                        }
                    }
                    newComponent.add(u);
                }
                components.add(newComponent);

        }

        int lost = N;
        for(int i = 0; i < components.size();i++){
            for(int j = 0; j < components.get(i).size();j++){
                if(components.get(i).get(j) + 1 == K){
                    lost = lost - components.get(i).size();
                    break;
                }
            }

        }
        System.out.println(lost);

        }catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }
    }
    public static void main(String args[]){
        readFile("C:\\Users\\hoando\\Downloads\\dataset_547244_2.txt");
    }

}
