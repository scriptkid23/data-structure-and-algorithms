package com.spirity.Dynamic;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class ATM {
    static int min(int a, int b){
        return a < b ? a : b;
    }
    public static void main(String args[]){
        try{

        String filename = "C:\\Users\\hoando\\Downloads\\datasets.txt";
        File myObj = new File(filename);
        Scanner myReader = new Scanner(myObj);
        int M=myReader.nextInt();
        int N=myReader.nextInt();
        int v[] = new int[N];
        for(int  i = 0; i < N; i++){
            v[i] = myReader.nextInt();
        }
//        int v[] = {1,389,1042,540,1950,1310,1440,416,420,680,49,1842,1463,697,186,1724,447,322,332,716,213,1753,730,1115,732,1500,1632,1504,1249,99,1638,122,1643,113,1524,1783,1528,250};

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
        System.out.println(F[2][3]);
        } catch (FileNotFoundException e){

        }

    }
}
