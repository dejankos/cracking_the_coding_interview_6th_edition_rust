use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter, Debug};
use std::fmt;
use std::hash::Hash;
use std::rc::Rc;

type RcVertex<T> = Rc<RefCell<Vertex<T>>>;

struct Graph<T>
{
    nodes: Vec<RcVertex<T>>
}

struct Vertex<T>
{
    v: T,
    adj_vertices: Vec<RcVertex<T>>,
    visited: bool,
}


impl<T> Graph<T>
where T: Debug
{
    fn new() -> Self {
        Graph {
            nodes: vec![]
        }
    }

    fn add_vertex(&mut self, v: RcVertex<T>) {
        self.nodes.push(v)
    }

    fn bfs(&mut self, v: RcVertex<T>) {
        let mut deq = VecDeque::new();
        v.borrow_mut().visited = true;
        deq.push_back(v.clone());

        while let Some(rc) = deq.pop_front() {
            println!("Visit {:?}", rc.borrow().v);

            for v in rc.borrow().adj_vertices.clone(){
                if !v.borrow().visited {
                    v.borrow_mut().visited = true;
                    deq.push_back(v.clone());
                }
            }
        }

        println!("LEN {}", deq.len())
    }
}

impl<T> Vertex<T> {
    fn new(v: T) -> RcVertex<T> {
        Rc::new(
            RefCell::new(
                Vertex {
                    v,
                    adj_vertices: vec![],
                    visited: false
                }
            )
        )
    }

    fn add_vertices(&mut self, v: RcVertex<T>) {
        self.adj_vertices.push(v)
    }
}


#[allow(unused_must_use)]
impl<T: Display> Display for Graph<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for n in self.nodes.clone() {
            write!(f, " {}: ", n.borrow().v);
            for v in n.borrow().adj_vertices.clone() {
                write!(f, " {}, ", v.borrow().v);
            }
            write!(f, "{}", "\n");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_a_direct_graph() {
        let mut g = Graph::new();

        let v_a = Vertex::new('a');
        let v_b = Vertex::new('b');
        let v_c = Vertex::new('c');
        let v_d = Vertex::new('d');
        let v_e = Vertex::new('e');
        let v_f = Vertex::new('f');

        v_a.borrow_mut().add_vertices(v_b.clone());
        v_a.borrow_mut().add_vertices(v_e.clone());
        v_a.borrow_mut().add_vertices(v_f.clone());
        v_b.borrow_mut().add_vertices(v_c.clone());
        v_c.borrow_mut().add_vertices(v_d.clone());

        g.add_vertex(v_a.clone());
        g.add_vertex(v_b.clone());
        g.add_vertex(v_c.clone());
        g.add_vertex(v_d.clone());

        println!("{}", g);

        g.bfs(v_a);
    }
}