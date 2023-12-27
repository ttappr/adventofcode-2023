/// Brent's Algorithm - finds the length of a cycle in a sequence and the start 
/// index of the cycle. The return value is of the form `(lam, mu)` where `lam` 
/// is the length of the cycle and `mu` is the start index of the cycle. 
/// 
/// The function takes a tuple, `(T, I)` for the initial value of the sequence, 
/// where `T` is the sequence value type and `I` is ignored by the algorithm. 
/// `I` can be used by the caller for whatever purpose is needed to associate 
/// some extra data with each value. This could be an index, map key, or 
/// whatever.
/// 
fn brent<F, T, I>(x0: (T, I), f: F) -> (usize, usize) 
where
    F: Fn(&(T, I)) -> (T, I),
    T: PartialEq + Clone,
    I: Clone,
{
    let mut power = 1;
    let mut lam   = 1;
    let mut tort  = x0.clone();
    let mut hare  = f(&x0);
    let mut mu    = 0;

    while tort.0 != hare.0 {
        if power == lam {
            power *= 2;
            tort   = hare.clone();
            lam    = 0;
        }
        hare = f(&hare);
        lam += 1;
    }
    tort = x0.clone();
    hare = x0.clone();

    for _ in 0..lam {
        hare = f(&hare);
    }
    while tort.0 != hare.0 {
        tort = f(&tort);
        hare = f(&hare);
        mu += 1;
    }
    (lam, mu)
}
