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

        return result;
    }

    return vec![];
}
