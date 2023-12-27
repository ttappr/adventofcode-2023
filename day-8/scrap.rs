
fn foo() {
    for key in ['a', 'b'] { //&curr {
        let (left, right) = map.get(key).ok_or("Bad map!")?;

        let (lam, mu) = {
            brent(key, |k| {
                let (l, r) = map.get(k).unwrap();

                if l.as_bytes()[2] == b'A' {
                    l.clone()
                } else {
                    r.clone()
                }
            })
        };
        // Get the vector that represents the cycle.
        let mut cycle = Vec::new();
        let mut k     = key.clone();

        for _ in 0..lam + mu {
            cycle.push(k.clone());
            k = map.get(&k).unwrap().1.clone();
        }

        println!("{}: lam = {}, mu = {}", key, lam, mu);
        //println!("a = {:?}", &cycle[mu..]);
    }
}