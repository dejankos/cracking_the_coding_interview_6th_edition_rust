// Build Order: You are given a list of projects and a list of dependencies (which is a list of pairs of
// projects, where the second project is dependent on the first project). All of a project's dependencies
// must be built before the project is. Find a build order that will allow the projects to be built. If there
// is no valid build order, return an error.
// EXAMPLE
// Input:
// projects: a, b, c, d, e, f
// dependencies: (a, d), (f, b), (b, d), (f, a), (d, c)
// Output: f, e, a, b, d, c

use std::collections::VecDeque;
use std::fmt::Debug;
use std::rc::Rc;

use crate::graph::{Graph, RcVertex};

fn topological_sort<T>(g: Graph<T>) -> Vec<RcVertex<T>>
where
    T: Debug,
{
    let mut deq = VecDeque::new();
    g.nodes
        .iter()
        .for_each(|n| r_topological_sort(n.clone(), &mut deq));

    deq.iter().map(|rc| rc.clone()).collect()
}

fn r_topological_sort<T>(v: RcVertex<T>, deq: &mut VecDeque<RcVertex<T>>)
where
    T: Debug,
{
    v.borrow().adj_vertices.clone().iter().for_each(|rc| {
        if !rc.borrow().visited {
            r_topological_sort(rc.clone(), deq);
            rc.borrow_mut().visited = true;
        }
    });

    if !contains(&deq, v.clone()) {
        deq.push_front(v.clone());
    }
}

fn contains<T>(v: &VecDeque<RcVertex<T>>, rc: RcVertex<T>) -> bool {
    v.iter().find(|v| Rc::ptr_eq(v, &rc)).is_some()
}

fn unwrap_value<T>(source: Vec<RcVertex<T>>) -> Vec<T>
where
    T: Copy,
{
    source.iter().map(|rc| rc.borrow().v).collect()
}

#[cfg(test)]
mod tests {
    use crate::graph::{Graph, Vertex};

    use super::*;
    

    #[test]
    fn should_sort_order() {
        let mut g = Graph::new();

        let v_a = Vertex::new('a');
        let v_b = Vertex::new('b');
        let v_c = Vertex::new('c');
        let v_d = Vertex::new('d');
        let v_e = Vertex::new('e');
        let v_f = Vertex::new('f');

        v_a.borrow_mut().add_vertices(v_d.clone());
        v_f.borrow_mut().add_vertices(v_b.clone());
        v_b.borrow_mut().add_vertices(v_d.clone());
        v_f.borrow_mut().add_vertices(v_a.clone());
        v_d.borrow_mut().add_vertices(v_c.clone());

        g.add_vertex(v_a.clone());
        g.add_vertex(v_b.clone());
        g.add_vertex(v_c.clone());
        g.add_vertex(v_d.clone());
        g.add_vertex(v_e.clone());
        g.add_vertex(v_f.clone());

        let sorted = topological_sort(g);
        assert_eq!(
            "['f', 'e', 'b', 'a', 'd', 'c']",
            format!("{:?}", unwrap_value(sorted))
        );
    }
}
