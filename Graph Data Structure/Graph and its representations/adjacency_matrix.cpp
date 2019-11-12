#include <iostream>
using namespace std;

void createGraph(int matrix[100][100],int node){
  for(int i = 0; i < node;i++){
    for(int j = 0; j < node;j++){
      matrix[i][j] = 0;
    }
  }
}
void updateWeight(int matrix[100][100], int node,int xVertex, int yVertex,int value){
  matrix[xVertex][yVertex] = value;
}
void showGraph(int matrix[100][100],int node){
  for(int i = 0; i < node; i++){
    for(int j = 0; j < node; j++){
      cout << matrix[i][j] << " ";
    }
    cout << endl;
  }
}
int main(int argc, char const *argv[]) {
  int graph[100][100];
  int node =  10;
  createGraph(graph,node);
  updateWeight(graph,node,5,3,1);
  updateWeight(graph,node,1,3,1);
  updateWeight(graph,node,3,3,1);
  updateWeight(graph,node,6,7,1);
  updateWeight(graph,node,5,3,1);
  showGraph(graph,node);



  return 0;
}
