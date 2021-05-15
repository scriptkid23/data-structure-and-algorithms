package com.spirity.Dynamic;

public class LengthMax {
    public static void main(String[] arg){
        int A[] = {1, 2, 5, 3, 4, 7, 6, 4, 9};
        int F[] = new int[A.length];
        int maxIndexF = 0;
        int track[] = new int[A.length];
        track[0] = -1;
        //track[i] = j tức là phần tử đứng trước i là phần tử j
        // nếu trước số 5 là  số 2 tương ứng với trước vị trí 2 có vị trí 1
        // track[2] = 1;
        // tính từ F[0] -> F[n-1]
        F[0] = 1; //
        int maxF = 0;

        for(int i = 1; i < A.length;i++){

            int maxFj = 0; // biểu diễn không tìm thấy j;
            int maxIndexJ = -1; // không tìn thấy
            for(int j = 0; j < i; j++){
                    if(A[i] > A[j]){
                        // khi A[i] > A[j] tức có thể nối vào chuỗi tăng dần
                        // tìm F[j] lớn nhất chạy từ 0 đến i
                        // tìm max của mảng F[j] với điều kiện A[i] > A[j]
                        if(F[j] > maxFj){
                            maxFj = F[j];
                            maxIndexJ = j;
                        }
                    }
            }
            // sau khi có được max của mảng thì chỉ cần so sánh max của mảng F[i] + 1 với 1
            // theo công thức max (1, maxFj + 1);

            if(maxFj > 0){
                F[i] = maxFj + 1; //
                track[i] = maxIndexJ ;
//                System.out.println(A[i]);
            }
            else{
                F[i] = 1; // bắt đầu 1 dãy con mới
//                System.out.println(A[i]);
                track[i] = -1;
            }

            // tìm phần tử max trong dãy F[i]
            if(maxF < F[i]){
                maxF = F[i];
//                System.out.println(A[i]);
                maxIndexF = i;
            }
        }
        System.out.print(maxF);
        // sử dụng kỹ thuật lưu vết để lần ra dãy dài nhất

        // truy vết
        int index = maxIndexF;
        while (index!= -1){
            System.out.print(A[index]+" ");
            index = track[index];
        }
    }
}
