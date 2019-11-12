#include "modules/graph.cpp"
// #include "queue.cpp"
#include <iostream>
using namespace std;

int main() {

   int V = 4;
   Graph* graph = createGraph(V);
   addEdge(graph, 0, 1);
    addEdge(graph, 0, 2);
    addEdge(graph, 1, 2);
    addEdge(graph, 2, 3);


    dfs(graph, 2);


   return 0;

}
