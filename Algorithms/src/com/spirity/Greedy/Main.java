package com.spirity.Greedy;

import java.util.ArrayList;
import java.util.Arrays;

public class Main {

    static int X = 289;
    static ArrayList<Integer> S = new ArrayList<>();
    static int C[] = {1, 5, 25, 100};

    static int Select(){
        for(int i =0; i < C.length; i++){
            if(C[i] <= X){
                return C[i];
            }
        }
        return 0;
    }
    public static void main(String[] arg){

        //Sap xep cac menh gia
        for(int i = 0; i < C.length - 1; i++){
            for(int j = i+1; j < C.length; j++){
                if(C[i] < C[j]){
                    int tg = C[i];
                    C[i] = C[j];
                    C[j] = tg;
                }
            }
        }
        while(X > 0){
            // them cac dong tien vao S;
            int k = Select();
            if(k == 0){
                System.out.println("Not found!");
                break;
            }
            else {
                X = X - k;
                S.add(k);
            }
        }
        for (int i = 0; i < S.size();i++){
            System.out.print(S.get(i)+" ");
        }


    }
}
