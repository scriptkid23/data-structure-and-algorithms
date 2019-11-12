#include <iostream>
using namespace std;
struct Node {
    int value;
    Node *next;
};
struct Queue {
    Node *front;
    Node *rear;
};

Node *createNode(int value){
    Node *newNode = new Node();
    newNode->next = NULL;
    newNode->value = value;
    return newNode;
}
Queue *createQueue(){
    Queue *newQueue = new Queue();
    newQueue->front = newQueue->rear = NULL;
    return newQueue;
}
void showQueue(Queue *queue){
    Node *temp = queue->rear;
    cout << "queue container" << ": ";
    while(temp != NULL){
        cout << " " << temp->value << " ";
        temp = temp->next;
    }
    cout << endl;
}

bool isQueueEmpty(Queue *queue){
    return ((queue->front == NULL)) ? true : false;
}
int deQueue(Queue *queue){

    Node *temp = queue->front;
    Node *temp_ = queue->rear;
    int items = queue->front->value;
    if(queue->front == queue->rear){
      queue->front = queue->rear = NULL;
      delete temp;
      return items;
    }
    while (temp_->next->next != NULL) {
      temp_ = temp_->next;
    }
    queue->front = temp_;

   queue->front->next = NULL;
   delete temp;
    return items;

}
void enQueue(Queue *queue,int value){
    Node *newNode = createNode(value);
    if(queue->rear == NULL){
        queue->front = queue->rear = newNode;
        return;
    }
    newNode->next = queue->rear;
    queue->rear = newNode;
}
