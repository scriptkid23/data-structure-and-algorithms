package com.spirity.Backtracking;

import java.util.ArrayList;
/*
    Bài toán mê cung: một ma trận mê cung map
    với 0 là các đường đi được, và 1 là các tường
    người chơi chỉ có thể đi sang phải, đi xuống và đi chéo
    sử dụng giải thuật quay lui để đi đến điểm có tọa độ (4,4)
 */
public class Main {
    static int map[][]={
            {0,0,0,1,0},
            {0,1,0,0,0},
            {0,0,0,1,0},
            {0,1,0,0,0},
            {0,0,1,0,0}
    };
    static Position X[];
    static int N=5;
    static int counter = 0;
    static ArrayList nextMoves(int k, Position X[], int map[][]){
        ArrayList moves = new ArrayList<Position>();
        Position rightPos = new Position(X[k-1].x + 1, X[k-1].y);
        if(rightPos.x < N && map[rightPos.x][rightPos.y] == 0){
            moves.add(rightPos);

        }
        Position downPos = new Position(X[k-1].x, X[k-1].y + 1);
        if(downPos.y < N && map[downPos.x][downPos.y] == 0){
            moves.add(downPos);

        }
        Position crossPos = new Position(X[k-1].x + 1, X[k-1].y + 1);
        if(crossPos.x < N && crossPos.y < N && map[crossPos.x][crossPos.y] == 0){
            moves.add(crossPos);
        }
        return moves;

    }
    static boolean isStop(int k, Position X[],int map[][]){ // điểm neo
        if(X[k].isEqual(new Position(N-1,N-1))){
            return true;
        }
        else{
            return false;
        }
    }
    static void processResult(int k, Position X[],int map[][]){ // kết quả

        for(int i = 0; i <= k; i++)
            System.out.print("("+X[i].x + "," + X[i].y+")");


        System.out.println();


    }
    static public void Try(int k){

        ArrayList<Position> moves=nextMoves(k,X,map);
        for(int i=0;i<moves.size();i++){
            X[k]=moves.get(i);
            if(isStop(k,X,map)){
                counter++;
                processResult(k,X,map);
            }
            else
                Try(k+1);
        }

    }
    public static void main(String[] args){
        Position start=new Position(0,0);

        X=new Position[100];
        X[0]=start;
        Try(1);
        System.out.println(counter);
    }
}
class Position{
    public int x;
    public int y;
    public Position(int x, int y){
        this.x = x;
        this.y=y;
    }
    public boolean isEqual(Position p){
        if(p.x==x&&p.y==y)
            return true;
        else
            return false;
    }
}
