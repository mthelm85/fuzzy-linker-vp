use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::levenshtein::levenshtein_distance;

type NodeRef = Option<Rc<RefCell<Node>>>;

pub struct Node {
    data: String,
    median: usize,
    left: NodeRef,
    right: NodeRef,
}

impl Node {
    fn new(data: String, median: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            data,
            median,
            left: None,
            right: None,
        }))
    }
}

pub struct VPTree {
    index_map: HashMap<String, usize>,
    root: NodeRef,
}

impl<'a> VPTree {
    pub fn new(data: &'a [String]) -> Self {
        let index_map = data.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
        let mut tree = VPTree { index_map, root: None };
        tree.root = tree.build_recursive(data);
        tree
    }

    fn build_recursive(&self, data: &[String]) -> NodeRef {
        if data.is_empty() {
            return None;
        }

        let (vantage_point, distances) = self.select_vantage_point(data);
        let median = self.calculate_median(&distances);

        let mut left_data = Vec::new();
        let mut right_data = Vec::new();

        for (i, &dist) in distances.iter().enumerate() {
            if i != vantage_point {
                if dist <= median {
                    left_data.push(data[i].clone());
                } else {
                    right_data.push(data[i].clone());
                }
            }
        }

        let node = Node::new(data[vantage_point].clone(), median);
        node.borrow_mut().left = self.build_recursive(&left_data);
        node.borrow_mut().right = self.build_recursive(&right_data);

        Some(node)
    }

    fn select_vantage_point(&self, data: &[String]) -> (usize, Vec<usize>) {
        let mut best_vantage_point = 0;
        let mut best_balance = std::usize::MAX;
    
        for (i, point) in data.iter().enumerate() {
            let distances = data.iter().map(|s| levenshtein_distance(point, s)).collect::<Vec<usize>>();
            let median = self.calculate_median(&distances);
            let left_data = distances.iter().enumerate().filter(|&(j, _)| j != i && distances[j] <= median).count();
            let right_data = distances.iter().enumerate().filter(|&(j, _)| j != i && distances[j] > median).count();
            let balance = (left_data as isize - right_data as isize).abs() as usize;
    
            if balance < best_balance {
                best_vantage_point = i;
                best_balance = balance;
            }
        }
    
        let distances = data.iter().map(|s| levenshtein_distance(&data[best_vantage_point], s)).collect();
        (best_vantage_point, distances)
    }

    fn calculate_median(&self, distances: &[usize]) -> usize {
        let mut distances = distances.to_vec();
        distances.sort();
        distances[distances.len() / 2]
    }

    pub fn search(&self, query: &String, threshold: usize) -> Vec<usize> {
        let mut result = Vec::new();
        self.search_recursive(&mut result, query, threshold, &self.root);
        result
    }
    
    fn search_recursive(&self, result: &mut Vec<usize>, query: &String, threshold: usize, node_ref: &NodeRef) {
        if let Some(node) = node_ref {
            let node = node.borrow();
            let dist = levenshtein_distance(query, &node.data);
    
            if dist <= threshold {
                let index = *self.index_map.get(&node.data).unwrap();
                result.push(index); // Push usize directly
            }
    
            if dist <= node.median + threshold {
                self.search_recursive(result, query, threshold, &node.left);
            }
    
            if dist >= node.median.saturating_sub(threshold) {
                self.search_recursive(result, query, threshold, &node.right);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let data = vec!["motherfucker".to_string(), "bull".to_string(), "shit".to_string(), "shite".to_string(), "shoot".to_string(), "shot".to_string()];
        let tree = VPTree::new(&data);
        assert_eq!(tree.root.is_some(), true);
        if let Some(root) = &tree.root {
            println!("Root: {}", root.borrow().data);
        }
    }

    #[test]
    fn test_search() {
        let data = vec!["bla".to_string(), "blub".to_string(), "asdf".to_string(), ":assd".to_string(), "ast".to_string(), "baube".to_string()];
        let tree = VPTree::new(&data);
        let result = tree.search(&"blau".to_string(), 3);
        assert_eq!(result, vec![5, 0, 1]);
        let result = tree.search(&"b".to_string(), 1);
        assert_eq!(result, Vec::<usize>::new());
    }
}