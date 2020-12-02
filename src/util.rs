pub fn both<T,U>(opt1: Option<T>, opt2: Option<U>) -> Option<(T,U)> {
    if opt1.is_none() {
        return None
    }

    if opt2.is_none() {
        return None
    }

    Some((opt1.unwrap(), opt2.unwrap()))
}

// copied from https://stackoverflow.com/questions/45786955/how-to-compose-functions-in-rust
pub fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

pub fn option_bind<T,U>(opt: Option<T>, f: fn(T) -> U) -> Option<U> {
    match opt {
        Some(val) => Some(f(val)),
        None => None,
    }
}
