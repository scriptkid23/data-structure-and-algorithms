package com.spirity.Dynamic;

import java.util.ArrayList;

public class WeightArray {
    static int a[] ={1, -2, 8, -3, -6, 15, -2, 9, 7, -1};
    static  int start = 0,end = 0, temp = 0;

    public static long process(int[] a){
            long max = a[0];
            int[] s = new int[a.length];
            s[0] = a[0];
            max = s[0];
            for(int i = 1; i < a.length; i++){
                if(s[i-1] > 0){
                    s[i] = s[i-1]+a[i];
                }else{
                    s[i] = a[i];
                    temp = i;
                }
                max = max > s[i] ? max : s[i];
                if(max < s[i]){
                    max = s[i];
                    start = temp;
                    end = i;

                }
            }
            return  max;
    }
    public static void main(String[] arg){
        System.out.println(process(a));
        System.out.print(start+" "+end);
    }
}
