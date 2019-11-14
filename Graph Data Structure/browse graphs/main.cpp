#include "modules/graph.cpp"
// #include "queue.cpp"
#include <iostream>


/*
==================================README==========================================
|- Vertex : đỉnh
|- Edge : cạnh
|
|- addEdge(Graph *param1,int param2,int param3) : tạo kết nối giữa 2 đỉnh
|- createGraph(int param1) : khởi tạo đồ thị
|- showGraph(Graph *param1) : hiển thị đồ thị theo dạng Adjacency List
|- dfs(Graph *param1,int param2): duyệt đồ thị theo Depth First Search
|- bfs(Graph *param1,int param2): duyệt đồ thị theo Breadth First Search
|- isVertexConnectedDFS(Graph *param1,int param2, int param3) : kiểm tra có đường đi
   từ 2 đỉnh hay không theo Depth First Search
|- isVertexConnectedBFS(Graph *param1,int param2, int param3) : kiểm tra có đường đi
   từ 2 đỉnh hay không theo Breadth First Search
|
==================================README==========================================
*/
using namespace std;

int main() {

   int V = 6;
   Graph* graph = createGraph(V);
   addEdge(graph, 0, 1);
   addEdge(graph, 0, 2);
   addEdge(graph, 1, 2);
   addEdge(graph,3,2);
   addEdge(graph,4,5);
   showGraph(graph);
   dfs(graph, 3);
   isVertexConnectedBFS(graph,3,2) ? cout << "true" : cout << "false";
   return 0;

}
