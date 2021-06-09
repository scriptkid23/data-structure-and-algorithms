package com.spirity.Greedy;

/*
    Hè này Y nhận được N lời rủ đi du lịch từ bạn bè, mỗi chuyến đi du lịch có thời gian bắt đầu là si và kết thúc là fi. .
    Tuy nhiên, nghỉ liên tục thì xin sếp cũng …hơi ngại,
    do vậy Y tự đặt ra quy tắc mỗi chuyến đi cách nhau ít nhất là M ngày.
    Hãy giúp Y sắp xếp sao cho có thể đi được nhiều chuyến du lịch nhất có thể.
 */

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class Practice1 {
//    static int N = 48, M = 2;
//    static int s[] = {1,48,82,84,33,22,6,38,19,41,14,21,56,37,54,71,31,77,38,28,94,34,64,96,68,76,77,27,57,83,69,44,36,43,49,38,71,94,79,79,77,76,46,50,56,61,96,63};
//    static int f[] = {94,51,97,92,81,65,46,56,88,48,79,24,95,61,83,72,58,80,54,82,96,42,81,98,77,85,77,38,97,83,84,44,40,90,76,56,98,99,84,92,94,80,66,93,83,75,98,81};

    public static void main(String args[]){
        try {
            File myObj = new File("C:\\Users\\hoando\\Downloads\\dataset_544963_1.txt");
            Scanner myReader = new Scanner(myObj);
            int N=myReader.nextInt();
            int M=myReader.nextInt();
            int s[]=new int[N];
            int f[]=new int[N];
            System.out.println("N:"+N);
            System.out.println("M:"+M);
            System.out.print("s[]:");
            for(int i=0;i<N;i++){
                s[i]=myReader.nextInt();
                System.out.print(s[i]+",");
            }
            System.out.println();
            System.out.print("f[]:");
            for(int i=0;i<N;i++){
                f[i]=myReader.nextInt();
                System.out.print(f[i]+",");
            }
            myReader.close();
            System.out.println();
            for(int i = 0; i < N-1; i++){
                for(int j = i + 1; j < N; j++ ){
                    if(f[i] > f[j]){
                        int temp = s[i];
                        s[i] = s[j];
                        s[j] = temp;

                        temp = f[i];
                        f[i] = f[j];
                        f[j] = temp;
                    }
                }
            }
            System.out.print("s sort: ");
            for(int i = 0; i < N; i ++){
                System.out.print(s[i]+" ");
            }
            System.out.println();
            System.out.print("f sort: ");
            for(int i = 0; i < N; i ++){
                System.out.print(f[i]+" ");
            }
            int pointer= 0,i = 1, count = 1;

                while (i<N){
                    if(f[pointer]+M <= s[i]){
                        pointer = i;
                        count++;

                    }
                    i++;
                }

            System.out.println();
            System.out.println("Result:"+count);
        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }


    }
}
