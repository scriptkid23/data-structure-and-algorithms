package com.spirity.Graph;

import java.util.Stack;

public class DFS {
    public static void main(String args[]){
        int N = 4;
        int A[][] ={
                {0,1,1,1},
                {1,0,1,0},
                {1,1,0,1},
                {1,0,1,0}

        };
        int d[] = new int[N];
        // duyệt cả đồ thị để tìm thành phần liên thông
        // do không dùng lệnh for thì chỉ duyệt đồ thị từ đỉnh 0
        // ví dụ như trong đồ thị có 2 đồ thị con khác nhau nhưng mỗi đồ thị con đó lại có thành phần liên thông

        for(int i = 0; i < N; i++){
            if(d[i] == 1) continue;
            Stack<Integer> S = new Stack<>();
            S.push(i);

            while (!S.empty()){
                int u = S.pop();
                d[u] = 1;
                System.out.print(u+1+" ");
                for(int v = 0; v < N;v++){
                    if(A[u][v] == 1 && d[v] == 0 && !S.contains(v)){
                        S.push(v);
                    }
                }

            }
            System.out.println();
        }

    }
}
