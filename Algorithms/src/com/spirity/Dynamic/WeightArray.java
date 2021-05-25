package com.spirity.Dynamic;


public class WeightArray {
    static int a[] ={-3,1,9,-2,-5,10,-3,3,-3};
    static int start = 0,end = 0, temp = 0;

    /*
        Xây dựng thuật toán
        Xét mảng a gồm n phần tử;
        Gọi S(n) là giá trị của dãy tuần tự có trọng số lớn nhất bắt buộc phải kết thúc tại phần tử n
        xét S(0) = 1 tồn tại 1 tập hợp max(sum(1)) = 1
        xét S(1) = -2 tồn tại 2 tập hợp max(sum(-2), sum(-2,1)) = -1
        xét S(2) = 8 tồn tại 3 tập hợp max(sum(8), sum(8,-2), sum(8,-2,1))
        ta thấy max(sum(8), sum(8,-2), sum(8,-2,1)) tương đương với max(sum(8), sum(8,-1)) vì ta có
        max(sum(8), sum(8,-2), sum(8,-2,1)) = max(max(sum(8)), max(sum(8,-2), sum(8,-2,1)))
        Đánh giá max(sum(8,-2), sum(8,-2,1)) ta thấy do max(sum(-2), sum(-2,1)) = -1 cộng cả 2 vế với 8
        => max(sum(8,-2), sum(8,-2,1)) = sum(8,-1) mà -1 = S(1)
        => S(2) = max(sum(8), sum(8,S(1))
        => công thức tổng quát S(k) = max(sum(a(k)), sum(a(k), S(k-1))
        nếu max = sum(a(k)) thì bắt đầu một dãy mới
        nếu max = sum(a(k), S(k-1)) thì nối giá trị a(k) với dãy cũ
     */

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
                System.out.print(s[i]+",");


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
