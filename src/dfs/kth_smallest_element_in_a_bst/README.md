# 230. Kth Smallest Element in a BST
![230 Kth Smallest Element in a BST.png](../../../images/230%20Kth%20Smallest%20Element%20in%20a%20BST.png)

Constraints:
- The number of nodes in the tree is n. 
- 1 <= k <= n <= 104

### Solution Step

1. 트리를 순회하며 노드의 값을 저장하는 스택 생성
2. 스택 정렬후 k번째 값을 반환