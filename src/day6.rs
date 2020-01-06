use crate::day::Day;
use crate::graph::{Graph};

pub struct Day6 {
}

impl Day for Day6 {
    fn first_puzzle(&self) -> String {
        let graph = self.read_graph();
        let com = String::from("COM");
        let mut count = 0;
        for v in graph.vertices().filter(|&v| *v != com) {
            let route = graph.find_route(&com, v).expect("every object indirectly orbits COM");
            count += route.len();
        }

        format!("{}", count)
    }

    fn second_puzzle(&self) -> String {

        let graph = self.read_graph();
        let san = String::from("SAN");
        let me = String::from("YOU");

        match graph.find_route(&me, &san) {
            Some(me_to_san) => return format!("{}", me_to_san.len() - 2),
            None => {}
        };

        match graph.find_route(&san, &me) {
            Some(san_to_me) => return format!("{}", san_to_me.len() - 2),
            None => {}
        };

        let mut min_trans = usize::max_value();
        for v in graph.vertices().filter(|&v| *v != san && *v != me) {
            let v_to_san = graph.find_route(&v, &san);
            if v_to_san.is_none() {
                continue;
            }

            let v_to_me = graph.find_route(&v, &me);
            if v_to_me.is_none() {
                continue;
            } 

            let trans = v_to_me.unwrap().len() + v_to_san.unwrap().len() - 2;
            min_trans = usize::min(min_trans, trans);
        }

        format!("{}", min_trans)
    }

    fn number(&self) -> u8 {
        6
    }
}

impl Day6 {
    fn read_graph(&self) -> Graph<String, usize> {
       
        let mut graph = Graph::new();
        for l in self.read_input_lines_string(&self.input()) {
            let mut it = l.split(')');
            let s = String::from(it.next().unwrap());
            let e = String::from(it.next().unwrap());
            graph.add_edge(s, e, 1);
        }

        graph
    }
}

