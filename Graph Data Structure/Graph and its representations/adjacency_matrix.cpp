#include <iostream>
using namespace std;

void CreateMatrix(int matrix[100][100],int nodes){
  for(int i = 0; i < nodes;i++){
    for(int j = 0; j < nodes;j++){
      cout << "graph[" << i <<"]" << "[" << j << "]:";
      cin >> matrix[i][j] ;
    }
  }
}

int main(int argc, char const *argv[]) {

  
  return 0;
}
