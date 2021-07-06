package com.spirity.Greedy;

import java.util.ArrayList;

public class Trongcay {
    static int a[] = {1,5,8,9,18,22,3,23,31,45};
    static int n = 10;
    public static void quickSort(int a[], int l, int r){
        int p = a[(l+r)/2];
        int i = l, j = r;
        while (i < j){
            while (a[i] < p){
                i++;
            }
            while (a[j] > p){
                j--;
            }
            if (i <= j){
                int temp = a[i];
                a[i] = a[j];
                a[j] = temp;
                i++;
                j--;
            }
        }
        if (i < r){
            quickSort(a, i, r);
        }
        if (l < j){
            quickSort(a, l, j);
        }
    }

    public static void main(String[] arg){
        quickSort(a, 0,  n-1);
        int k = 1;
        int max = 1;

        for(int i = n -1; i >= 0; i--){
           a[i] += k++;
           if(max < a[i]){
               max = a[i];
           }
        }
        max++;
        System.out.println(max);
    }
}
