// Given a directed graph, design an algorithm to find out whether there is a
// route between two nodes.
use crate::graph::{contains, Graph, RcVertex};

fn has_route(graph: &mut Graph<char>, from: RcVertex<char>, to: RcVertex<char>) -> bool {
    let path = graph.bfs(from.clone());

    contains(&path, to.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{Graph, Vertex};

    #[test]
    fn should_have_a_route() {
        let mut g = Graph::new();

        let v_a = Vertex::new('a');
        let v_b = Vertex::new('b');
        let v_c = Vertex::new('c');
        let v_d = Vertex::new('d');
        let v_e = Vertex::new('e');

        v_a.borrow_mut().add_vertices(v_b.clone());
        v_a.borrow_mut().add_vertices(v_e.clone());
        v_b.borrow_mut().add_vertices(v_c.clone());
        v_c.borrow_mut().add_vertices(v_d.clone());

        g.add_vertex(v_a.clone());
        g.add_vertex(v_b.clone());
        g.add_vertex(v_c.clone());
        g.add_vertex(v_d.clone());
        g.add_vertex(v_e.clone());

        assert!(has_route(&mut g, v_a.clone(), v_d.clone()));
    }
}
