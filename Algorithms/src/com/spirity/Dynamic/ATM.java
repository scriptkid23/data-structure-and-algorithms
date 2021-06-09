package com.spirity.Dynamic;

public class ATM {
    static int min(int a, int b){
        return a < b ? a : b;
    }
    public static void main(String args[]){
        int v[] = {1,3,15,20,30,50};

        int M = 63;
        int infinity = M+1;
        int F[][] = new int[v.length+1][infinity];

        for(int i = 1; i <= M; i++) F[0][i] = infinity;
        for(int i = 1; i <=v.length;i++) F[i][0] = 0;

        for(int i = 1; i <= v.length; i++){
            for(int j = 1; j<=M;j++){
                if(j <v[i-1]){
                    F[i][j] = F[i-1][j];
                }
                else {
                    F[i][j] = min(F[i-1][j],F[i][j-v[i-1]]+1);
                }
            }
        }
        if(F[v.length][M] == M + 1){
            System.out.println("Khong the thanh toan");
            return;
        }
//        for(int i = 0; i <= v.length; i++){
//            for(int j = 0; j <=M;j++){
//                System.out.println(F[i][j]);
//            }
//        }
        System.out.println("So to tien can rut: "+F[v.length][M]);
        int i = v.length, j = M, c = 1;
        while (j > 0){
            if(F[i][j] < F[i-1][j]){
                System.out.println("Voi menh gia: "+v[i - 1]);
                j = j -v[i - 1];
                c = c + 1;
            }
            else{
                i = i - 1;
            }
        }

    }
}
