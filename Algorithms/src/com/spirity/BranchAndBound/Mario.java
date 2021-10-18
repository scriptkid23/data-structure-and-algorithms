package com.spirity.BranchAndBound;

import java.util.ArrayList;

public class Mario {
    static int map[]={0,0,0,1,0,0,1,0,0,1,0};
    static int X[]; //cấu hình lưu trữ số bước nhảy
    static int M = 10;
    static int best=Integer.MAX_VALUE;
    static int K1 = 1;
    static int K2 = 2;
    static int K3 = 3;
    static ArrayList nextMoves(int k, int X[], int map[]){
        ArrayList<Integer> moves=new ArrayList<Integer>();

        // tinh toa do hien tai

        int S = 0;
        for(int i = 0; i < k; i++){
            S = S + X[i];
        }
        if(S+K1 <= M && map[S+K1] == 0){
            moves.add(K1);
        }
        if(S+K2 <= M & map[S+K2] == 0){
            moves.add(K2);
        }
        if(S+K3 <= M && map[S+K3] == 0){
            moves.add(K3);
        }

        return moves;

    }
    static boolean isStop(int k, int X[]){
        int  S = 0;
        for(int i = 0; i <=k; i++){
            S+=X[i];
        }
        if(S == M){
            return true;
        }
        else{
            return false;
        }
    }
    static void processResult(int k, int X[]){
        for(int i=0;i<=k;i++)
            System.out.print(X[i]+",");
        System.out.println();
    }
    static int z_k(int k,int X[]){
        int s = 0;
        // tinh do dai doan da di qua
        for(int i = 0; i < k; i++){
            s+=X[i];
        }
        // tinh do dai
        int next_s = k + (M-s)/K3;
        return next_s;
    }
    static int z(int k, int X[]){
        int count = 1;
        for(int i = 0; i < k ;i++){
            count++;
        }
        return count;
    }
    static public void Try(int k){
        ArrayList<Integer> moves=nextMoves(k,X,map);
        for(int i=0; i<moves.size();i++){
            X[k]=moves.get(i);
            if(isStop(k,X)){
                processResult(k,X);
                int d = z(k,X);
                System.out.println("Do dai: "+d);
                System.out.println("Best:"+best);
                if(d < best)
                    best = d;

            }
            else
            if(z_k(k,X) < best){
                Try(k+1);
            }
        }

    }
    public static void main(String[]args){
        X=new int[100];
        Try(0);
        System.out.println("Best="+best);

    }
}
