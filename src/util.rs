use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct OneToManySortedMap<K: Clone + Eq + Hash, V: Clone + Ord> {
    pub map: HashMap<K, Vec<V>>,
}

impl<K: Clone + Eq + Hash, V: Clone + Ord> OneToManySortedMap<K,V> {
    pub fn new() -> OneToManySortedMap<K,V> {
        OneToManySortedMap {
            map: HashMap::<K,Vec<V>>::new()
        }
    }

    pub fn remove(&mut self, key: &K) {
        self.map.remove(key);
    }

    pub fn insert(&mut self, key: K, val: V) {
        self.map.entry(key)
            .and_modify(|many| {
                many.push(val.clone());
                many.sort();
            })
            .or_insert(vec![val]);
    }
}


pub fn both<T,U>(opt1: Option<T>, opt2: Option<U>) -> Option<(T,U)> {
    if opt1.is_none() {
        println!("opt1 none");
        return None
    }

    if opt2.is_none() {
        println!("opt2 none");
        return None
    }

    Some((opt1.unwrap(), opt2.unwrap()))
}

// copied from https://stackoverflow.com/questions/45786955/how-to-compose-functions-in-rust
// pub fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
// where
//     F: Fn(A) -> B,
//     G: Fn(B) -> C,
// {
//     move |x| g(f(x))
// }

pub fn option_bind<T,U>(opt: Option<T>, f: fn(T) -> Option<U>) -> Option<U> {
    match opt {
        Some(val) => f(val),
        None => None,
    }
}

pub fn sublists<T: Clone>(iter: &mut dyn Iterator<Item = T>) -> Vec<(Vec<T>,Vec<T>)> {
    let elem = iter.next();

    if let Some(elem) = elem {
        let mut result = Vec::new();

        let rest = sublists(iter);

        // add all the sublists with this element
        for (with, without) in &rest {
            let mut with = with.clone();
            with.push(elem.clone());
            result.push((with, without.clone()));
        }

        // add all the sublists without this element
        for (with, without) in &rest {
            let mut without = without.clone();
            without.push(elem.clone());
            result.push((with.clone(), without));
        }

        // add the cases with only this element to consider
        result.push((vec![elem.clone()], vec![]));
        result.push((vec![], vec![elem]));

        return result;
    }

    return vec![];
}

pub trait SumI64 {
    fn sum(&mut self) -> i64;
}

impl SumI64 for dyn Iterator<Item = i64> {
    fn sum(&mut self) -> i64 {
        self.fold(0, |acc, next| acc + next)
    }
}

pub trait ProductI64 {
    fn product(&mut self) -> i64;
}

impl ProductI64 for dyn Iterator<Item = i64> {
    fn product(&mut self) -> i64 {
        self.fold(1, |acc, next| acc * next)
    }
}
