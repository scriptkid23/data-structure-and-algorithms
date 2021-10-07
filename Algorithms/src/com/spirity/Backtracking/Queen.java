package com.spirity.Backtracking;

import java.util.ArrayList;

public class Queen {
    // yêu cầu xếp quân hậu thỏa mãn không có 2 quân hậu ăn được nhau.

    // khai báo vị trí của các quân hậu từ hàng thứ 0 -> 5
    static int count = 0;
    static int n = 8;
    // Khai báo hàm Try : là một hàm trong kỹ thuật thực hiện quay lui
    // Hàm thử đánh giá tại vị trí thứ k
    // Giả sử đặt được k - 1 quân hậu, bây giờ ta tìm cách đặt quân hậu trên hàng k
    // và quy nạp với cách đặt quân hậu tại hàng k + 1
    static public void Try(int Q[], int k){
        // kiểm tra điểm đặt quân hậu thứ k
        boolean legal = false;

        // thỏa mãn điều kiện kết thúc với vị trí đặt hậu vượt quá n, in kết quả ra màn hình
        if(k == n + 1){
            count++;

                // print Q[];
            System.out.println();

            for(int i = 1; i <= n;i++){

                System.out.print(Q[i]+" ");

            }
                return;
        }
        // xét cột j với hàng thứ k
        for(int j = 1; j <= n;j++){
            legal = true;
            for(int i = 1; i <= k - 1  ;i++){
                // Kiểm tra các quân hậu ăn nhau,
                // để tìm kiếm vị trí đặt hậu tại hàng thứ k
                // nên kiểm tra vị trí đặt tại bước k với từng quân hậu đã được đặt trước đó
                // tức kiểm tra Q[1] -> Q[k-1]
                // Q[i] == j Kiểm tra quân hậu đặt tại hàng i theo cột j
                // Q[i] == k - i + j xét tréo trái của quân hậu hàng thứ i với bước thứ k
                // Q[i] == i - k + j xét tréo phải của quân hậu hàng thứ i với bước thứ k
                // k - i + j xét tréo trái vì k - i tại đây luôn dương về phía trái của quân hậu trước đó
                // nên chỉ cần cộng với cột j
                // i - k luôn âm nên khi
                if(Q[i] == j || Q[i] == j + k - i || Q[i] == j-k+i){
                    legal = false;
                }
            }
            // đệ quy chỉ được thực hiện khi có một giá trị mới
            // được thêm vào tập lời giải.
            if(legal){
                Q[k] = j;
                Try(Q,k+1);
            }
        }

    }


    public static void main(String[] args){
        int Q[] = new int[n+1];
        // Xét k = 1 ta có:
        Try(Q,1);
//        Q[1] = 1;

//        boolean legal = false;
//        int k = 2;
//        for(int j = 1; j <= n;j++){
//            legal = true;
//            for(int i = 1; i <= 3 - 1  ;i++){
//                System.out.print(",Q[i] = "+Q[i]);
//                System.out.print(",j = "+j);
//                System.out.print(",j + k - i = "+(j + k - i));
//                System.out.print(",j-k+i = "+(j-k+i));
//                if(Q[i] == j || Q[i] == j + k - i || Q[i] == j-k+i){
//                    legal = false;
//                }
//            }
//            if(legal){
//               System.out.println("Trigger");
//            }
//            else{
//                System.out.println("No Trigger");
//            }
//
//        }
//
//        for(int i = 0; i <0;i++){
//            System.out.println(Q[1]);
//        }
        System.out.println();
        System.out.println(count);

    }
}
