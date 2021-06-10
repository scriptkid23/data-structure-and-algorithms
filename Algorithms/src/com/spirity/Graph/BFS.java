package com.spirity.Graph;

import java.util.LinkedList;
import java.util.Queue;

public class BFS {
    public static void main(String args[]){
        int N = 7;
        int A[][] ={

                {0,1,1,0,0,0,0},
                {1,0,1,0,0,0,1},
                {1,1,0,0,1,0,1},
        {0,1,0,0,0,1,1},
        {0,0,1,0,0,1,1},
        {0,0,0,1,1,0,0},
        {0,1,1,1,1,0,0}
        };
        int d[] = new int[N];
        for(int i = 0; i < N; i++){
            if(d[i] != 0) continue;
            else{
                Queue<Integer> Q = new LinkedList<>();
                Q.add(0); // thêm đỉnh 0 vào trước
                d[0] = 1; // đánh dấu đỉnh 0 đã thăm

                while (!Q.isEmpty()){
                    int u = Q.poll();
                    System.out.print(u+1+",");
                    for(int v = 0; v < N; v++){
                        if(A[u][v] == 1 && d[v] == 0){
                            Q.add(v);
                            d[v] = 1;  // đánh dấu đỉnh v đã thăm

                        }

                    }
                }
            }
        }


    }

}
