#include <iostream>
#include <stdlib.h>
using namespace std;

struct AdjListNode {
  int dest;  // destination
  AdjListNode *next;

};
struct AdjList {
  AdjListNode *head;
};

struct Graph {
  int Vertex;
  AdjList *array;
};

struct AdjListNode *newAdjListNode(int dest){
  AdjListNode *newNode = new AdjListNode();
  newNode->dest = dest;
  newNode->next = NULL;
  return newNode;
}
struct Graph *createGraph(int Vertex){
  Graph *graph = new Graph();
  graph->Vertex = Vertex;
  graph->array = new AdjList[Vertex];

  for(int i = 0 ; i < Vertex;++i){
    graph->array[i].head = NULL;
  }
  return graph;
}
void addEdge(Graph *graph,int src,int dest){
  AdjListNode* newNode = newAdjListNode(dest);
  newNode->next = graph->array[src].head;
  graph->array[src].head = newNode;

  newNode = newAdjListNode(src);
  newNode->next = graph->array[dest].head;
  graph->array[dest].head = newNode;

}
void showGraph(Graph* graph)
{
    int v;
    for (v = 0; v < graph->Vertex; ++v)
    {
        AdjListNode* temp = graph->array[v].head;
        cout << endl <<"Adjacency list of node " << v << " : " ;
        while (temp != NULL)
        {
            cout << temp->dest;
            if(temp->next != NULL){
              cout << "->";
            }
            temp = temp->next;
        }
        printf("\n");
    }
}

int main(int argc, char const *argv[]) {

   int Vertex = 5;
   Graph* graph = createGraph(Vertex);
   addEdge(graph, 0, 1);
   addEdge(graph, 0, 4);
   addEdge(graph, 1, 2);
   addEdge(graph, 1, 3);
   addEdge(graph, 1, 4);
   addEdge(graph, 2, 3);
   addEdge(graph, 3, 4);


   showGraph(graph);

   return 0;

}
