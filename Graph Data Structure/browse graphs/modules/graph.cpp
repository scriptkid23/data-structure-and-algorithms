#include <iostream>
#include "queue.cpp"
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
  int *visited;
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
  graph->visited = new int[V];
  for(int i = 0 ; i < V;++i){
    graph->array[i].head = NULL;
    graph->visited[i] = 0;
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
    for (v = 0; v < graph->V; ++v)
    {
        AdjListNode* pCrawl = graph->array[v].head;
        cout << endl <<"Adjacency list of node " << v << endl <<  "head";
        while (pCrawl)
        {
            cout << "->" << pCrawl->dest;
            pCrawl = pCrawl->next;
        }
        cout << endl;
    }
}
void bfs(Graph *graph,int startVertex){
  Queue *queue = createQueue();
  graph->visited[startVertex] = 1;
  enQueue(queue,startVertex);
  while (!isQueueEmpty(queue))
  {
    showQueue(queue);
    int currentVertex = deQueue(queue);
    cout << "visited: " << currentVertex << endl;
    AdjListNode *temp = graph->array[currentVertex].head;
    while (temp != NULL)
    {
      int adjVertex = temp->dest;
      if(graph->visited[adjVertex] == 0){
        graph->visited[adjVertex] = 1;
        enQueue(queue,adjVertex);
      }
      temp = temp->next;
    }
  }
}
void dfs(Graph *graph, int startVertex){
  AdjListNode *node = graph->array[startVertex].head;
  AdjListNode *temp = node;

  graph->visited[startVertex] = 1;
  cout << "visited: " << startVertex << endl;
  while (temp != NULL) {
    /* code */
    int connectedVertex = temp->dest;
    if(graph->visited[connectedVertex] == 0) {
      dfs(graph,connectedVertex);
    }
    temp = temp->next;
  }
}
bool isVertexConnectedDFS(Graph *graph,int startVertex,int endVertex){
  AdjListNode *node = graph->array[startVertex].head;
  AdjListNode *temp = node;

  graph->visited[startVertex] = 1;
  if(graph->visited[endVertex] == 1){
    return true;
  }else{
  while (temp != NULL) {
    /* code */
    int connectedVertex = temp->dest;
    if(graph->visited[connectedVertex] == 0) {
      dfs(graph,connectedVertex);
    }
    temp = temp->next;
  }
}
}
bool isVertexConnectedBFS(Graph *graph, int startVertex,int endVertex){
  Queue *queue = createQueue();
  graph->visited[startVertex] = 1;
  if(graph->visited[endVertex] == 1){
    return true;
  }
  else{
  enQueue(queue,startVertex);
  while (!isQueueEmpty(queue))
  {
    showQueue(queue);
    int currentVertex = deQueue(queue);
    cout << "visited: " << currentVertex << endl;
    AdjListNode *temp = graph->array[currentVertex].head;
    while (temp != NULL)
    {
      int adjVertex = temp->dest;
      if(graph->visited[adjVertex] == 0){
        graph->visited[adjVertex] = 1;
        enQueue(queue,adjVertex);
      }
      temp = temp->next;
    }
  }
}
}
