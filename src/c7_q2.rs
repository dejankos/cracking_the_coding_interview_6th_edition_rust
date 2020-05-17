// Call Center: Imagine you have a call center with three levels of employees: respondent, manager,
// and director. An incoming telephone call must be first allocated to a respondent who is free. If the
// respondent can't handle the call, he or she must escalate the call to a manager. If the manager is not
// free or not able to handle it, then the call should be escalated to a director. Design the classes and
// data structures for this problem. Implement a method dispatchCall() which assigns a call to
// the first available employee.
trait CallCenter {
    fn new(emps: Vec<Box<dyn Employee>>) -> Self;
    fn dispatch_call(&mut self) -> Option<&Box<dyn Employee>>;
}

trait Employee {
    fn new(assigned: bool) -> Self
    where
        Self: Sized;
    fn is_free(&self) -> bool;
    fn assign(&mut self);
    fn lvl(&self) -> EmpLevel;
}

struct Respondent {
    assigned: bool,
}

struct Manager {
    assigned: bool,
}

struct Director {
    assigned: bool,
}

struct CCenter {
    emps: Vec<Box<dyn Employee>>,
}

#[derive(Debug, Eq, PartialEq)]
enum EmpLevel {
    Respondent,
    Manager,
    Director,
}

impl EmpLevel {
    fn emp_lvl(&self) -> u8 {
        match *self {
            EmpLevel::Director => 3,
            EmpLevel::Manager => 2,
            EmpLevel::Respondent => 1,
        }
    }
}

impl Employee for Respondent {
    fn new(assigned: bool) -> Self {
        Respondent { assigned }
    }

    fn is_free(&self) -> bool {
        !self.assigned
    }

    fn assign(&mut self) {
        self.assigned = true
    }

    fn lvl(&self) -> EmpLevel {
        EmpLevel::Respondent
    }
}

impl Employee for Manager {
    fn new(assigned: bool) -> Self {
        Manager { assigned }
    }

    fn is_free(&self) -> bool {
        !self.assigned
    }

    fn assign(&mut self) {
        self.assigned = true
    }

    fn lvl(&self) -> EmpLevel {
        EmpLevel::Manager
    }
}

impl Employee for Director {
    fn new(assigned: bool) -> Self {
        Director { assigned }
    }

    fn is_free(&self) -> bool {
        !self.assigned
    }

    fn assign(&mut self) {
        self.assigned = true
    }

    fn lvl(&self) -> EmpLevel {
        EmpLevel::Director
    }
}

impl CallCenter for CCenter {
    fn new(mut emps: Vec<Box<dyn Employee>>) -> Self {
        emps.sort_by(|x, y| x.lvl().emp_lvl().cmp(&y.lvl().emp_lvl()));
        CCenter { emps }
    }

    fn dispatch_call(&mut self) -> Option<&Box<dyn Employee>> {
        let mut emp = self.emps.iter_mut().find(|e| e.is_free());

        if let Some(e) = emp {
            e.assign();
            Some(e)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_dispatch_call_in_order() {
        let mut emps: Vec<Box<dyn Employee>> = vec![
            Box::new(Director { assigned: false }),
            Box::new(Manager { assigned: false }),
            Box::new(Respondent { assigned: false }),
        ];

        let mut cc = CCenter::new(emps);
        let res = cc.dispatch_call();
        assert!(res.is_some());
        assert_eq!(EmpLevel::Respondent, res.unwrap().lvl());

        let res = cc.dispatch_call();
        assert!(res.is_some());
        assert_eq!(EmpLevel::Manager, res.unwrap().lvl());

        let res = cc.dispatch_call();
        assert!(res.is_some());
        assert_eq!(EmpLevel::Director, res.unwrap().lvl());

        let res = cc.dispatch_call();
        assert!(res.is_none());
    }
}
