#include <iostream>

using namespace std;

struct Node {
  int value;
  Node *next;
};
struct Stack {
  Node *top;
};

Stack *createStack(){
    Stack *stack = new Stack();
    stack->top = NULL;
    return stack;
}
void pop(Stack *stack,int value){
  Node *newNode = new Node();
  newNode->value = value;
  newNode->next = stack->top;
  stack->top = newNode;
}
void showStack(Stack *stack){
  Node *temp = stack->top;
  while (temp != NULL) {
    cout << temp->value;
    if(temp->next != NULL ) cout << " ";
    temp = temp->next;
  }
  cout << endl;
  cout << "^" << endl;
  cout << "top";
}

int main(int argc, char const *argv[]) {

  Stack *stack = createStack();
  pop(stack,5);
  pop(stack,6);
  pop(stack,8);
  pop(stack,2);
  pop(stack,1);
  showStack(stack);
  return 0;
}
