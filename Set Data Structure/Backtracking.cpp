#include<iostream>
// tại thời điểm t = 1
// (1) ta có try(1)
// (2) lấy 1 giá trị trong tập kết quả {0,1} => {1}
// (3) gán giá trị vào vị trì thứ 1

// nếu độ dài > 2 thì dừng lại 

// (4) ta có try(2)
// (5) lấy 1 giá trị trong tập kết quả {0,1} => {1}
// (6) gán giá trị vào vị trí thứ 2

// nếu độ dài >2 thì dưng lại 

// (7) lấy 1 gía trị trong tập kết quả của bước 5 hay {1} => {}

int x[2];
void try(int t) {
    if(t > 2) {
        cout << x[0] << x[1] << endl;
        return;
    }
    for(int i = 0; i <=1;i++){
        x[t] = i;
        try(t+1);
    }


}

int main(int argc, char const *argv[])
{
    try(1);
    return 0;
}
