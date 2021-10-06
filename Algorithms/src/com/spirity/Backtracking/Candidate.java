package com.spirity.Backtracking;
import java.util.ArrayList;
import java.lang.Math;

// Bài toán con của bài toán N con hậu.
public class Candidate {
    public static int N = 8;
    public static ArrayList candidates(int k,int X[]){
        ArrayList<Integer> list_candidate = new ArrayList<>();
        for(int v = 0; v <= N -1; v++){
            if(k == 0) list_candidate.add(v);
            else{
                boolean not_conflict = true;
                for(int i = 0; i <= k -1;i++){
                    if(v == X[i] || Math.abs(v - X[i]) == Math.abs(k-i)){
                        not_conflict = false;
                    }
                }
                if(not_conflict) list_candidate.add(v);
            }
        }
        return list_candidate;
    }
    public static void main(String[] args) {
        int X[] = {0,2};
        System.out.println(candidates(2,X));
    }
}
