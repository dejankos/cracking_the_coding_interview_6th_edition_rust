use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

pub type RcVertex<T> = Rc<RefCell<Vertex<T>>>;

pub struct Graph<T> {
    pub nodes: Vec<RcVertex<T>>,
}

pub struct Vertex<T> {
    pub v: T,
    pub adj_vertices: Vec<RcVertex<T>>,
    pub visited: bool,
}

impl<T> Graph<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        Graph { nodes: vec![] }
    }

    pub fn add_vertex(&mut self, v: RcVertex<T>) {
        self.nodes.push(v)
    }

    pub fn bfs(&mut self, v: RcVertex<T>) -> Vec<RcVertex<T>> {
        let mut deq = VecDeque::new();
        let mut visited = Vec::new();
        v.borrow_mut().visited = true;
        deq.push_back(v.clone());

        while let Some(rc) = deq.pop_front() {
            visited.push(rc.clone());
            rc.borrow().adj_vertices.clone().iter().for_each(|rc| {
                if !rc.borrow().visited {
                    rc.borrow_mut().visited = true;
                    deq.push_back(rc.clone());
                }
            });
        }

        visited
    }
}

impl<T> Vertex<T> {
    pub fn new(v: T) -> RcVertex<T> {
        Rc::new(RefCell::new(Vertex {
            v,
            adj_vertices: vec![],
            visited: false,
        }))
    }

    pub fn add_vertices(&mut self, v: RcVertex<T>) {
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

pub fn contains<T>(v: &Vec<RcVertex<T>>, rc: RcVertex<T>) -> bool {
    v.iter().find(|v| Rc::ptr_eq(v, &rc)).is_some()
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

        v_a.borrow_mut().add_vertices(v_b.clone());
        v_a.borrow_mut().add_vertices(v_e.clone());
        v_b.borrow_mut().add_vertices(v_c.clone());
        v_c.borrow_mut().add_vertices(v_d.clone());

        g.add_vertex(v_a.clone());
        g.add_vertex(v_b.clone());
        g.add_vertex(v_c.clone());
        g.add_vertex(v_d.clone());
        g.add_vertex(v_e.clone());

        let expected = " a:  b,  e, \n b:  c, \n c:  d, \n d: \n e: \n";
        assert_eq!(expected, format!("{}", g))
    }

    #[test]
    fn should_have_a_path() {
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

        let path = g.bfs(v_c.clone());

        assert!(contains(&path, v_c.clone()));
        assert!(contains(&path, v_d.clone()));
    }

    #[test]
    fn should_not_have_a_path() {
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

        let path = g.bfs(v_b.clone());
        assert!(!contains(&path, v_a.clone()));
    }
}
