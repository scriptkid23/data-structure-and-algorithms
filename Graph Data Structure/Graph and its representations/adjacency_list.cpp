#include <iostream>
#include <stdlib.h>
using namespace std;

struct AdjListNode {
  int dest;
  AdjListNode *next;

};
struct AdjList {
  AdjListNode *head;
};

struct Graph {
  int V;
  AdjList *array;
};

struct AdjListNode *newAdjListNode(int dest){
  AdjListNode *newNode = new AdjListNode();
  newNode->dest = dest;
  newNode->next = NULL;
  return newNode;
}
struct Graph *createGraph(int V){
  Graph *graph = new Graph();
  graph->V = V;
  graph->array = new AdjList[V];

  for(int i = 0 ; i < V;++i){
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
void printGraph(Graph* graph)
{
    int v;
    for (v = 0; v < graph->V; ++v)
    {
        AdjListNode* pCrawl = graph->array[v].head;
        cout << endl <<"Adjacency list of node " << v << endl <<  "head";
        while (pCrawl)
        {
            cout << "->" << pCrawl->dest;
            pCrawl = pCrawl->next;
        }
        printf("\n");
    }
}

int main(int argc, char const *argv[]) {

   int V = 5;
   Graph* graph = createGraph(V);
   addEdge(graph, 0, 1);
   addEdge(graph, 0, 4);
   addEdge(graph, 1, 2);
   addEdge(graph, 1, 3);
   addEdge(graph, 1, 4);
   addEdge(graph, 2, 3);
   addEdge(graph, 3, 4);

   // print the adjacency list representation of the above graph
   printGraph(graph);

   return 0;

}
