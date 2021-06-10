package com.spirity.Dynamic;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class practice2 {
    static int start = 0,end = 0, temp = 0;


    public static long process(int[] a){
        long max = a[0];

        int[] s = new int[a.length];
        s[0] = a[0];
        max = s[0];

        for(int i = 1; i < a.length; i++){

            if(s[i-1] > 0){
                s[i] = s[i-1]+a[i];

            }else{
                // khởi tạo điểm xuất phát mới
                s[i] = a[i];
                // bắt điểm xuất phát mới
                temp = i;
            }
            if(max < s[i]){
                max = s[i];
                start = temp;
                end = i;
            }
        }
        return  max;
    }
    public static void readFile(String filename){
        try{
            File myObj = new File(filename);
            Scanner myReader = new Scanner(myObj);
            int N=myReader.nextInt();
            int K=myReader.nextInt();
            int a[] = new int[N];
            System.out.println("N:"+N);
            System.out.println("K:"+K);
            System.out.print("a[]:");
            for(int i=0;i<N;i++){
                a[i]=myReader.nextInt();
                a[i] = a[i] - K;
                System.out.print(a[i]+",");
            }
            System.out.println();
            myReader.close();
            System.out.println();
            System.out.println(process(a));
            System.out.print(start+" "+end);

        }catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }

    }
    public static void main(String[] arg){
        readFile("C:\\Users\\hoando\\Downloads\\dataset_540653_3.txt");


    }
}
