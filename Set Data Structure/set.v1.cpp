#include <iostream>

using namespace std;

struct SetNode {
  int value;
  SetNode *next;
};

struct Set{
  SetNode *head;
};

SetNode *createNode(int value){
  SetNode *node = new SetNode();
  node->value = value;
  node->next = NULL;
  return node;
}
Set *createSet(){
  Set *newSet = new Set();
  newSet->head = NULL;
  return newSet;
}
void addElementSet(Set *set,int value){
  SetNode *newNode = createNode(value);
  if(set->head == NULL){
      set->head = newNode;
      return;
  }
   newNode->next = set->head;
   set->head = newNode;

}
void showSet(Set *set,string setName){
  SetNode *temp = set->head;
  cout << setName << ": ";
  cout << "[";
  while (temp != NULL)
  {
    /* code */
    cout <<temp->value;
    if(temp->next != NULL){
      cout << ",";
    }
    temp = temp->next;
  }
  cout << "]" << endl;
}
Set *Union(Set *set1, Set *set2){
  SetNode *headOfSet1 = set1->head;
  SetNode *headOfSet2 = set2->head;
  Set *setUnion = set1;
  while (headOfSet1->next != NULL)
  {
    headOfSet1 = headOfSet1->next;
  }
  headOfSet1->next = headOfSet2;
  return setUnion;
}
int main(int argc, char const *argv[]) {

  Set *set1 = createSet();
  Set *set2 = createSet();
  Set *set3 = createSet();
  // create value list for set1
  addElementSet(set1,1);
  addElementSet(set1,3);
  addElementSet(set1,5);
  addElementSet(set1,1);
  addElementSet(set1,6);
  showSet(set1,"set1");
  // create value list for set2
  addElementSet(set2,5);
  addElementSet(set2,1);
  addElementSet(set2,7);
  addElementSet(set2,8);
  addElementSet(set2,9);
  showSet(set2,"set2");
  
  set3 = Union(set1,set2);
  showSet(set3,"set3");
  return 0;
}
