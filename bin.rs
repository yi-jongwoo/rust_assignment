// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

// A container storing a set of values, using a binary tree.
//
// If the same value is added multiple times, it is only stored once.
pub type BinaryTree<T:Ord> = Subtree<T>;

// Implement `new`, `insert`, `len`, and `has`.
impl<T:Ord> BinaryTree<T>{
    fn new()->BinaryTree<T>{
        Subtree{0:None}
    }
    fn insert(self:&mut BinaryTree<T>,item:T)->usize{
        if let Some(v)=&mut self.0{
            if (*v).value<item{
                (*v).left.insert(item);
            }else{ if (*v).value>item{
                (*v).right.insert(item);
            }}
        }else{
            self.0=Some(Box::new(Node::<T>{
                value:item,
                left:Subtree{0:None},
                right:Subtree{0:None}
            }));
        }
        return 0;
    }
    fn len(self:&BinaryTree<T>)->usize{
        if let Some(v)=&self.0{
            1+(*v).left.len()+(*v).right.len()
        }else{
            0
        }
    }
    fn has(self:&BinaryTree<T>,item:&T)->bool{
        if let Some(v)=&self.0{
            if (*v).value<*item{
                (*v).left.has(item)
            }else{ if (*v).value>*item{
                (*v).right.has(item)
            }else{
                true
            }}
        }else{
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();

        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();

        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> = (0..exp.len()).map(|val| tree.has(&(val as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();

        for i in 0..100 {
            tree.insert(i);
        }

        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}
