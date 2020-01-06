use std::collections::HashMap;
use std::iter::{Iterator};

pub type Edge<'a, Vertex, Weight> = (&'a Vertex, &'a Vertex, &'a Weight);

pub struct Graph<Vertex: PartialEq, Weight> {
    id: usize,
    vertices: HashMap<usize, Vertex>,
    edges: Vec<(usize, usize, Weight)>
}

impl<Vertex, Weight> Graph<Vertex, Weight> where Vertex: PartialEq {
    pub fn new() -> Graph<Vertex, Weight> {
        Graph{id: 0, vertices: HashMap::new(), edges: Vec::new()}
    }

    pub fn add_edge(&mut self, start: Vertex, end: Vertex, weight: Weight) {

        let start_vert = self.add_vert_priv(start);
        let end_vert = self.add_vert_priv(end);
        self.add_edge_priv(start_vert, end_vert, weight);
    }

    pub fn find_edge<'g>(&'g self, start: &Vertex, end: &Vertex) -> Option<Edge<'g, Vertex, Weight>> {

        let start_vert =  self.find_vert_priv(start);
        let end_vert = self.find_vert_priv(end);
        match (start_vert, end_vert) {
            (Some(sv), Some(ev)) => {
                self.find_edge_priv(sv, ev)
            }
            _ => None
        }
    }

    pub fn find_route<'g>(&'g self, start: &Vertex, end: &Vertex) -> Option<Vec<Edge<'g, Vertex, Weight>>> {

        let start_vert =  self.find_vert_priv(start);
        let end_vert = self.find_vert_priv(end);

        match (start_vert, end_vert) {
            (Some(sv), Some(ev)) => {
                self.find_route_priv(sv, ev)
            }
            _ => None
        }
    }

    pub fn vertices(&self) -> VertIter<Vertex> {
        VertIter {inner_iter: self.vertices.iter()}
    }

    pub fn edges<'g>(&'g self) -> EdgeIter<'g, Vertex, Weight> {
        EdgeIter{graph: self, inner_iter: self.edges.iter()}
    }

    fn add_vert_priv(&mut self, v: Vertex) -> usize {

        match self.find_vert_priv(&v) {
            Some(id) => id,
            None => {
                self.id += 1;
                self.vertices.insert(self.id, v);
                self.id
            }
        }
    }

    fn add_edge_priv(&mut self, start: usize, end: usize, weight: Weight) {
        self.edges.push((start,end, weight))
    }

    fn find_vert_priv(&self, v: &Vertex) -> Option<usize> {
        match self.vertices.iter().find(|(_, vert)| *vert == v) {
            Some((k, _)) => Some(*k),
            None => None
        }
    }

    fn get_vert_priv(&self, id: usize) -> &Vertex {
        self.vertices.get(&id).expect(&format!("not existing id: {}", id))
    }

    fn find_edge_priv<'g>(&'g self, start: usize, end: usize) -> Option<Edge<'g, Vertex, Weight>> {
        match self.edges.iter().find(|(s, e, _)| *s == start && *e == end) {
            Some((s, e, w)) => Some((self.get_vert_priv(*s), self.get_vert_priv(*e), &w)),
            None => None
        }
    }

    fn find_route_priv<'g>(&'g self, start: usize, end: usize) -> Option<Vec<Edge<'g, Vertex,Weight>>> {

        match self.find_edge_priv(start, end) {
            Some(direct) => return Some(vec![direct]),
            None => {}
        }

        let mut route = Vec::new();
        for (s, e, w) in self.edges.iter().filter(|(s, _, _)| *s == start) {
            match self.find_route_priv(*e, end) {
                Some(mut sub_route) => {
                    route.push((self.get_vert_priv(*s), self.get_vert_priv(*e), w));
                    route.append(&mut sub_route);
                    break; 
                },
                None => {}
            }
        }

        if route.len() != 0 {
            Some(route)
        } else {
            None
        }
    }
} 

pub struct VertIter<'a, Vertex: PartialEq> {
    inner_iter: std::collections::hash_map::Iter<'a, usize, Vertex>
}

impl<'a, Vertex> Iterator for VertIter<'a, Vertex> where Vertex: PartialEq {
    type Item = &'a Vertex;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner_iter.next() {
            Some((_, v)) => Some(v),
            None => None
        }
    }
}


pub struct EdgeIter<'a, Vertex, Weight> where Vertex : PartialEq {
    graph: &'a Graph<Vertex, Weight>,
    inner_iter: std::slice::Iter<'a, (usize, usize, Weight)>
}

impl<'a, Vertex, Weight> Iterator for EdgeIter<'a, Vertex, Weight> where Vertex: PartialEq {
    type Item = Edge<'a, Vertex, Weight>;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner_iter.next() {
            Some((s, e, w)) => Some((self.graph.get_vert_priv(*s), self.graph.get_vert_priv(*e), w)),
            None => None
        }
    }
}



#[cfg(test)]
mod tests 
{
    use super::*;

    static INPUT: [&str; 11] = [ 
                "COM)B",
                "B)C",
                "C)D",
                "D)E",
                "E)F",
                "B)G",
                "G)H",
                "D)I",
                "E)J",
                "J)K",
                "K)L"];

    fn build_graph() -> Graph<String, usize> {
        let mut graph = Graph::new();
        for i in INPUT.iter() {
            let tokens: Vec<_> = i.split(")").collect();
            let (s, e) = (String::from(tokens[0]), String::from(tokens[1]));
            graph.add_edge(s, e, 1);
        }
        graph
    }

    #[test]
    fn add_edge() {

        let mut graph: Graph<String, usize> = Graph::new();
        graph.add_edge(String::from("A"), String::from("B"), 1);
        graph.add_edge(String::from("B"), String::from("C"), 1);

        assert_eq!(graph.vertices.len(), 3);
        assert_eq!(graph.edges.len(), 2);
    }

    #[test]
    fn find_edge() {

        let (a, b, c) = (String::from("A"), String::from("B"), String::from("C"));

        let mut graph: Graph<String, usize> = Graph::new();
        graph.add_edge(a.clone(), b.clone(), 1);
        graph.add_edge(b.clone(), c.clone(), 1);

        let e1 = graph.find_edge(&a, &b);
        let e2 = graph.find_edge(&b, &c);
        let e3 = graph.find_edge(&a, &c);

        assert_eq!(e1.is_some(), true);
        assert_eq!(e2.is_some(), true);
        assert_eq!(e3.is_some(), false);
    }

    #[test]
    fn find_route() {
        let graph = build_graph();

        let (com, b, d, i, l) = (
            String::from("COM"),
            String::from("B"),
            String::from("D"),
            String::from("I"),
            String::from("L"),
        );

        let r1 = graph.find_route(&com, &b);
        assert!(r1.is_some());
        assert_eq!(r1.unwrap().len(), 1);

        let r2 = graph.find_route(&com, &d);
        assert!(r2.is_some());
        assert_eq!(r2.unwrap().len(), 3);

        let r3 = graph.find_route(&com, &i);
        assert!(r3.is_some());
        assert_eq!(r3.unwrap().len(), 4);

        let r4 = graph.find_route(&com, &l);
        assert!(r4.is_some());
        assert_eq!(r4.unwrap().len(), 7);

        assert!(graph.find_route(&d, &b).is_none());
        assert!(graph.find_route(&i, &l).is_none());
    }


    #[test]
    fn vertices() {
        let (a, b, c) = (String::from("A"), String::from("B"), String::from("C"));

        let mut graph: Graph<String, usize> = Graph::new();
        graph.add_edge(a.clone(), b.clone(), 1);
        graph.add_edge(b.clone(), c.clone(), 1);

        let mut it = graph.vertices();

        let valid = |v: &String| -> bool {
            *v == a || *v ==b || *v == c
        };

        assert!(valid(it.next().unwrap()));
        assert!(valid(it.next().unwrap()));
        assert!(valid(it.next().unwrap()));
        assert!(it.next().is_none());
    }

    #[test]
    fn edges() {
        let (a, b, c) = (String::from("A"), String::from("B"), String::from("C"));

        let mut graph: Graph<String, usize> = Graph::new();
        graph.add_edge(a.clone(), b.clone(), 1);
        graph.add_edge(b.clone(), c.clone(), 1);

        let mut it = graph.edges();

        assert_eq!(it.next().unwrap(), (&a, &b, &1));
        assert_eq!(it.next().unwrap(), (&b, &c, &1));
        assert!(it.next().is_none());
    }


}
