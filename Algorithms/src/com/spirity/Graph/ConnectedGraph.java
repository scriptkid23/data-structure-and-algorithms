package com.spirity.Graph;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.Queue;

public class ConnectedGraph {
    public static void main(String args[]){
        int N = 6;
        int A[][] = {
                {0,1,1,0,0,0},
                {1,0,1,0,0,0},
                {1,1,0,0,0,0},
                {0,0,0,0,0,0},
                {0,0,0,0,0,1},
                {0,0,0,0,1,0}
        };
        // thanh phan lien thong
        // Mang chua mang cac thanh phan lien thong
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
        System.out.println("Number:"+components.size());
        for(int i = 0; i < components.size();i++){
            System.out.print("Connected[" + i+"]: ");
            for(int j = 0; j < components.get(i).size();j++){
                System.out.print(components.get(i).get(j) + 1 +" ");
            }
            System.out.println();
        }



    }
}
