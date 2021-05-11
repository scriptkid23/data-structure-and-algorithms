package com.spirity.BranchAndBound;

/*
    Bài toán người du lịch sử dụng nhánh cận:
    trong thuật toán nhánh cận chúng ta cần xây dựng thêm 2 làm
    z_k và z

 */

import java.util.ArrayList;

public class Travelman {
    static int best = Integer.MAX_VALUE; // do dai duong di tot nhat hien tai
    static int X[];
    static int cmin = Integer.MAX_VALUE;

    static int cost[][]={
            {0,0,0,0,0},
            {0,0,3,7,4},
            {0,3,0,1,2},
            {0,7,1,0,5},
            {0,4,2,5,0}
    };
    static int z(int X[]){
        // tinh do dai 1 chu trinh: day cac thanh pho
        // duyet qua N+1 trong X
        int path = 0;
        for(int i = 0; i < 4; i++){
            path+=cost[X[i]][X[i+1]];
        }
        return path;
    }

    static int z_k(int k, int X[]){
        // danh gia do dai tot nhat ma cau hinh X hien tai co the dat duoc
        int path = 0;
        // tinh do dai doan da di qua
        for(int i = 0; i < k; i++){
            path+=cost[X[i]][X[i+1]];
        }
        // tinh do dai
        int next_path = (4-k)*cmin;
        return path + next_path;
    }

    static ArrayList nextMoves(int k, int X[], int cost[][]){
        //liet ke thanh pho tiep theo cua cau hinh X o buoc k
        ArrayList<Integer> moves=new ArrayList<Integer>();
        //thanh pho cuoi cung trong X: X[k-1]
        //duyet qua tat ca cac thanh pho
        for(int i=1;i<=4;i++){
            //kiem tra thanh pho i co canh noi voi X[k-1] hay khong
            if(cost[X[k-1]][i]>0){
                //neu co canh noi, thi kiem tra i da di qua chua
                //duyet qua tat ca cac thanh pho trong X
                boolean kt=false;
                for(int j=0;j<k;j++){
                    if(X[j]==i)
                        kt=true;//da di qua roi
                }
                if(!kt)//neu chua di qua i, them i vao danh sach ung vien
                    moves.add(i);
            }
        }
        //truong hop dax biet: khi da qua tat ca cac thanh pho => quay ve 1
        if(k==4)//k bat dau tu 0, neu k=N-1 nghia la da di qua het cac thanh pho
            moves.add(1);
        return moves;
    }

    static boolean isStop(int k){
        //kiem tra xem cau hinh da ket thuc hay chua
        //ket thuc khi da di qua het cac thanh pho => k=4
        if(k==4)
            return true;
        else
            return false;
    }
    static void processResult(int X[]){
        for(int i=0;i<=4;i++)
            System.out.print(X[i]+",");
        System.out.println();
    }
    static public void Try(int k){
        ArrayList<Integer> moves=nextMoves(k,X,cost);
        for(int i=0; i<moves.size();i++){
            X[k]=moves.get(i);
            if(isStop(k)){
                processResult(X);
                int d = z(X);
                System.out.println("Do dai: "+d);
                if(d < best)
                    best = d;

            }
            else
            if(z_k(k,X) < best){
                Try(k+1);

            }
        }

    }
    public static void main(String[] args){

        X=new int[100];
        for(int i = 1; i <=4;i++){
            for(int j = 1; j<=4;j++){
                if(cost[i][j] < cmin && cost[i][j] > 0){
                    cmin = cost[i][j];
                }
            }
        }
        X[0]=1;
        Try(1);
        System.out.println("Best="+best);
    }
}
