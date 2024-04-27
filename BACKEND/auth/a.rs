use std::collections::HashMap;

#[derive(Debug)]
struct NodeList {
    val: i32,
    next: Option<Box<NodeList>>
}

impl NodeList {
    fn new(v: i32) -> Self {
        Self{val: v, next: None}
    }
}

struct Solution;

impl Solution {
    fn frequencyCount(head: Option<Box<NodeList>>) -> Option<Box<NodeList>> {
        if head.is_none() {
            return head;
        }
        
        let mut dico: HashMap<i32, i32> = HashMap::new();
        
        let mut decoy = head;
        
        while let Some(v) = decoy {
            *dico.entry(&v.val).or_insert(0) += 1;
            decoy = v.next;
        }
        
        let mut current = NodeList::new(0);
        
        for (_k, v) in &dico {
            current.next = Some(Box::new(NodeList::new(*v)));
        }
        
        current.next
    }
}
