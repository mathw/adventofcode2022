use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    List(Vec<Value>),
    Integer(u32),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::List(l), Value::List(r)) => compare_lists(&mut l.iter(), &mut r.iter()),
            (Value::List(l), Value::Integer(r)) => {
                compare_lists(&mut l.iter(), &mut vec![Value::Integer(*r)].iter())
            }
            (Value::Integer(l), Value::List(r)) => {
                compare_lists(&mut vec![Value::Integer(*l)].iter(), &mut r.iter())
            }
            (Value::Integer(l), Value::Integer(r)) => l.cmp(r),
        }
    }
}

fn compare_lists<'a>(
    left: &mut impl Iterator<Item = &'a Value>,
    right: &mut impl Iterator<Item = &'a Value>,
) -> Ordering {
    match (left.next(), right.next()) {
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (Some(lv), Some(rv)) => match lv.cmp(rv) {
            Ordering::Equal => compare_lists(left, right),
            o => o,
        },
        (None, None) => Ordering::Equal,
    }
}
