#include <iostream>

using namespace std;

class Node
{
public:
    int key;
    Node *left;
    Node *right;

public:
    Node(int key)
    {
        this->key = key;
    }
};

void insertNode(Node **head, int key)
{
    if (*head == NULL)
    {
        Node *newNode = new Node(key);
        *head = newNode;
        return;
    }
    if ((*head)->key <= key)
    {
        insertNode(&((*head)->right), key);
    }
    if ((*head)->key > key)
    {
        insertNode(&((*head)->left), key);
    }
}

int main(int argc, char const *argv[])
{
   
    // ta co mojt mang gom 10: 9, 0, 1,4,7,3,2,1,10,9,11
    Node *head = NULL;
    insertNode(&head, 1);
    insertNode(&head, 3);
    insertNode(&head, 0);
    insertNode(&head, 2);
    cout << head->right->left->key;

    return 0;
}
