package com.spirity.Dynamic;

import java.util.ArrayList;

// khai báo đối tượng đồ vật
class Obj{
    public int name;
    public int weight;
    public int value;

    public Obj(int name, int weight, int value) {
        this.name = name;
        this.weight = weight;
        this.value = value;
    }
}

public class Knapsack {
    public static void main(String[] args){
        int N = 6;
        int L = 5;
        int weight[] = {1,2,4,3,5,4};
        int value[] = {3,4,3,7,3,2};
        ArrayList<Obj> list = new ArrayList<>();

        for(int i = 1; i <= N;i++){
            list.add(new Obj(i,weight[i-1],value[i-1]));
        }
        // khời tạo danh sách đồ vật

        int [][] F = new int[N+1][L+1];
        for(int i = 0; i <= L; i++) F[0][i] = 0;
        for(int i = 0; i <= N; i++) F[i][0] = 0;

        for(int k = 1; k<=N; k++){
            for(int S = 1; S<=L;S++){
                int wk = list.get(k-1).weight;
                int vk = list.get(k-1).value;
                // phải trừ 1 vì tìm đồ vật đầu tiên
                if(S < wk){
                    F[k][S] = F[k-1][S];
                }
                else{
                    // thực hiện hàm max
                    if(F[k-1][S] < F[k-1][S-wk]+vk){
                        F[k][S] = F[k-1][S-wk]+vk;
                    }
                    else{
                        F[k][S] = F[k-1][S];
                    }
                }
            }
        }
        /*
            để truy vết ta nhận thấy
            khi thêm đồ vật mới vào túi chỉ cần dò giá trị của F thay đổi chỗ nào thì chỗ đó sẽ là túi mới đã thêm
            dựa vào
         */
        // danh sách đồ vật được chọn
        ArrayList<Obj> pickedObjs = new ArrayList<>();
        // tính các giá trị nhỏ nhất của F
        System.out.println("Max :"+F[N][L]);
        // truy vết dựa vào trọng lượng còn lại
        // đưa ra được đáp án đúng nhưng không đưa ra được hết các đáp án đúng
        int k = N;
        int S = L;
        while (k!=0 || S != 0){
            if(F[k][S]!=F[k-1][S]){
                // đồ vật thứ k được chọn
                pickedObjs.add(list.get(k-1));
                S = S - list.get(k-1).weight;
                k--;
            }
            else{
                k=k-1;
            }
        }
        System.out.print("Select list: ");
        for(int i = 0; i<pickedObjs.size();i++){
            if(i == pickedObjs.size() - 1){
                System.out.print(pickedObjs.get(i).name);
            }
            else{
                System.out.print(pickedObjs.get(i).name+", ");
            }
        }
        System.out.println();
        for(int i = 0; i < N+1;i++){
            for(int j = 0; j < L+1; j++){
                System.out.println("F["+i+"]["+j+"]= "+F[i][j]);
            }
        }
    }
}

