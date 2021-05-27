package com.spirity.Dynamic;

import java.util.ArrayList;

/*
    Longest Common Substring
 */
public class LongestCommonSubstring {
    static String X = "ATCCGATC";
    static String Y = "CTTACGCT";
    static int maxLen  = 0;
    static  int endingIndex = X.length();
    static int F[][] = new int[X.length()+1][Y.length()+1];
    public static int max(int a, int b){
        if(a >= b) return a;
        else return b;
    }
    public static void main(String args[]) {

        for(int i = 1; i <= X.length();i++){
            for(int j = 1; j <= Y.length();j++){
                if(X.charAt(i-1) == Y.charAt(j-1)){
                    F[i][j]= F[i-1][j-1] + 1;

                }
                else{
                   F[i][j] = max(F[i-1][j],F[i][j-1]);
                }
            }
        }
        // truy vết dựa vào sự thay đổi giữa các giá trị F[i][j]
        // bằng cách di chuyển con trỏ truy vết theo quy tắc lên trên -> sang trái -> chéo bắt đầu từ F[X.length()][Y.length()]

        int i = X.length(), j = Y.length();
        String c = "";
        while (i != 0 && j != 0){
            // so sánh với giá trị cột
            //
            if(F[i][j] == F[i-1][j]){
                i = i - 1; // nhảy i lên trên
            }
            else if(F[i][j] == F[i][j - 1]){
                j = j - 1; // nhảy sang trái
            }
            else{
//                F[i][j] = F[i-1][j-1] // nhảy chéo bước này xác định sự thay đổi giữa các giá trị F[i][j]

                c = X.charAt(i-1)+c;
                i = i - 1;
                j = j - 1;
            }
        }
        System.out.println("Day con chung co do dai: "+F[X.length()][Y.length()]);
        System.out.println("Day con sau khi truy vet: "+c);

        int x = 5, y = 6;
        System.out.println("F["+x+"]["+y+"]: "+F[x][y]);
    }
}
