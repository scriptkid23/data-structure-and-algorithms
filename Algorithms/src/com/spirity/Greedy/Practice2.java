package com.spirity.Greedy;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

/*
Một người đi du lịch muốn mua một số loại quả về làm quà cho bạn bè và người thân.
Anh ta ghé vào một cửa hàng hoa quả nhỏ ven đường, ở đó bày bán N loại quả khác nhau,
mỗi loại quả đóng vào một túi, mỗi túi i nặng wi kg và có giá pi . Với mong muốn mua các loại quả “xịn” nhất,
anh ta muốn mua các loại quả đắt tiền nhất, nhưng trọng lượng hành lý lại bị giới hạn là W kg.
Hãy giúp anh ấy mua được số quả sao cho có tổng giá trị cao nhất mà không vượt quá W kg,
biết mỗi loại quả anh ta có thể mua lẻ đến 1 kg.
 */
public class Practice2 {
    public static void main(String args[]){
        try {
            File myObj = new File("C:\\Users\\hoando\\Downloads\\dataset_544963_2.txt");
            Scanner myReader = new Scanner(myObj);
            int N=myReader.nextInt();
            int W=myReader.nextInt();
            int w[]=new int[N];
            int v[]=new int[N];
            for(int i=0;i<N;i++){
                w[i]=myReader.nextInt();
                System.out.print(w[i]+",");
            }
            System.out.println();
            for(int i=0;i<N;i++){
                v[i]=myReader.nextInt();
                System.out.print(v[i]+",");
            }
            myReader.close();
            float g[] = new float[N];

            for(int i = 0; i< N; i++){
                g[i] = v[i]/w[i];
            }
            for(int i = 0; i < N - 1;i++){
                for(int j = i+1; j < N; j++){
                    if(g[i] < g[j]){
                        int temp = w[i];
                        w[i] = w[j];
                        w[j] = temp;

                        temp = v[i];
                        v[i] = v[j];
                        v[j] = temp;

                        float _temp = g[i];
                        g[i] = g[j];
                        g[j] = _temp;
                    }
                }
            }
            for(int i = 0; i < N;i++){
                System.out.print(w[i]+" ");
            }
            System.out.println();
            int pointer = 0;
            float sum = 0;
            while (true){
                if(W > w[pointer]) {
                    W = W - w[pointer];
                    sum = sum +v[pointer];
                    pointer++;

                }
                if(w[pointer] >= W){
                    sum = sum + W*g[pointer];
                    W = W - w[pointer];
                    break;

                }
            }
            System.out.println("Sum: "+sum);

        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }

//        int N = 20, W = 75;
//        int w[] = {36,72,56,63,79,5,10,20,96,2,31,10,18,29,25,8,8,40,47,76};
//        int v[] = {72,648,504,252,158,25,60,40,384,14,124,90,126,174,150,40,16,240,188,228};


    }
}
