package com.spirity.Graph;

import java.util.LinkedList;
import java.util.Queue;

public class ShortedPathBFS {
    public static void main(String args[]) {
        int N = 6;
        int A[][] = {
                {0,0,1,0,0,1},
                {0,0,1,0,1,1},
                {1,0,0,0,0,0},
                {0,1,0,0,0,1},
                {1,0,1,0,0,1},
                {0,1,0,1,0,1}
        };
        int d[] = new int[N];
        int h[] = new int[N];
        // h[i] chi ra khoang cach giua dinh i va dinh goc 0
        d[0] = 1;
        h[0] = 0; // khoang cach tu dinh 0 den dinh 0 = 0

        Queue<Integer> Q = new LinkedList<>();
        Q.add(0);
        while (!Q.isEmpty()){
            int u = Q.poll();
            System.out.print(u+1+",");
            for(int v = 0; v < N; v++){
                if(A[u][v] == 1 && d[v] == 0){
                    Q.add(v);
                    h[v] = h[u] + 1;
                    d[v] = 1;  // đánh dấu đỉnh v đã thăm

                }

            }
        }
        System.out.println();
        System.out.print("0,");
        for(int i = 1; i < N;i++){
            System.out.print(h[i]+",");
        }

    }
}
