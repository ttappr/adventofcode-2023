//! Brent's Algorithm - Online Version. An instance of `BrentOnline` can be used
//! to find the length of a cycle in a sequence and the start index of the 
//! cycle.
//! 
//! The object does store a portion of the series internally. So in most cases
//! this module will be suitable; however, if series are analyzed that have 
//! very large cycles, some significant memory could be used depending on the 
//! value type. Once a cycle is found it immediately drops its stored memory and
//! returns the cycle information.
//! 

use std::fmt::Debug;

/// Represents the internal state of the cycle detector.
/// 
#[derive(Debug, PartialEq, Clone)]
enum Stage<T> {
    InitTort,
    InitHare,
    First,
    FirstAwait((T, usize)),
    Second,
    Third,
    ThirdAwait((T, usize), (T, usize)),
    Done,
}

/// An online version of Brent's Algorithm - Create an instance and use the 
/// `.add()` method to add items to the series as they're encountered by the 
/// client code. 
/// 
/// Assuming the series fed to the object has a cycle, `add()` will eventually 
/// return an `Option` containing a tuple that holds the length of a cycle in 
/// the sequence and the start index of the cycle. The tuple is of the form
/// `(lam, mu)` where `lam` is the length of the cycle and `mu` is the start 
/// index of the cycle. 
/// 
/// The `.get_cycle() -> Option<(usize, usize)>` method can also be used to get 
/// the cycle information after the online object has found a cycle and is in 
/// the `Done`` state.
/// 
#[derive(Debug, Clone)]
pub struct BrentOnline<T> {
    x0     : Option<(T, usize)>,
    power  : usize,
    lam    : usize,
    mu     : usize,
    tort   : Option<(T, usize)>,
    hare   : Option<(T, usize)>,
    series : Option<Vec<T>>,
    stage  : Stage<T>,
}

impl<T> BrentOnline<T>
where
    T: PartialEq + Clone + Debug,
{
    /// Create a new instance of the cycle detector.
    /// 
    pub fn new() -> Self {
        Self {
            x0      : None,
            power   : 1,
            lam     : 1,
            mu      : 0,
            tort    : None,
            hare    : None,
            series  : Some(Vec::new()),
            stage   : Stage::InitTort,
        }
    }
    /// Reset the object to its initial state so new points of a series can
    /// be added.
    /// 
    pub fn reset(&mut self) {
        self.x0     = None;
        self.power  = 1;
        self.lam    = 1;
        self.mu     = 0;
        self.tort   = None;
        self.hare   = None;
        self.series = Some(Vec::new());
        self.stage  = Stage::InitTort;
    }
    /// If the object has found a cycle, this will return a tuple of the form
    /// `(lam, mu)` where `lam` is the length of the cycle and `mu` is the
    /// start index of the cycle. Otherwise, `None` is returned.
    /// 
    pub fn get_cycle(&self) -> Option<(usize, usize)> {
        if matches!(self.stage, Stage::Done) {
            Some((self.lam, self.mu))
        } else {
            None
        }
    }
    /// Add an item to the series. If the object has found a cycle, this will
    /// return a tuple of the form `(lam, mu)` where `lam` is the length of the
    /// cycle and `mu` is the start index of the cycle. Otherwise, `None` is
    /// returned. It will keep returning `None` until a cycle is found.
    /// 
    pub fn add(&mut self, x: T) -> Option<(usize, usize)> {
        use Stage::*;
        self.push(x.clone());

        loop {
            match &self.stage {
                InitTort => {
                    self.tort  = Some((x.clone(), 0));
                    self.x0    = self.tort.clone();
                    self.stage = InitHare;
                    break;
                },
                InitHare => {
                    self.hare  = Some((x.clone(), 1));
                    self.stage = First;
                    break;
                }
                First => {
                    let tort = self.tort.as_ref().unwrap();
                    let hare = self.hare.as_ref().unwrap();

                    if tort.0 != hare.0 {
                        if self.power == self.lam {
                            self.power *= 2;
                            self.tort   = self.hare.clone();
                            self.lam    = 0;
                        }
                        if let h @ Some(_) = self.get_next(&hare) {
                            self.hare = h;
                            self.lam += 1;
                        } else {
                            self.stage = FirstAwait(hare.clone());
                        }
                    } else {
                        self.stage = Second;
                    }
                },
                FirstAwait(hare) => {
                    if let h @ Some(_) = self.get_next(hare) {
                        self.hare  = h;
                        self.lam  += 1;
                        self.stage = First;
                    } else { break; }
                },
                Second => {
                    let value  = self.get(self.lam).clone();
                    self.hare  = Some((value, self.lam));
                    self.tort  = self.x0.clone();
                    self.stage = Third;
                },
                Third => {
                    let tort = self.tort.as_ref().unwrap();
                    let hare = self.hare.as_ref().unwrap();

                    if tort.0 != hare.0 {
                        if let (t @ Some(_), h @ Some(_)) 
                            = (self.get_next(tort), self.get_next(hare)) {
                                self.tort = t;
                                self.hare = h;
                                self.mu  += 1;
                        } else {
                            self.stage = ThirdAwait(tort.clone(), 
                                                    hare.clone());
                        }
                    } else {
                        self.stage = Done;
                    }
                },
                ThirdAwait(tort, hare) => {
                    if let (t @ Some(_), h @ Some(_)) 
                        = (self.get_next(tort), self.get_next(hare)) {
                            self.tort  = t;
                            self.hare  = h;
                            self.stage = Third;
                    } else { break; }
                },
                Done => {
                    self.series = None;
                    return Some((self.lam, self.mu))
                },
            }
        }
        None
    }
    /// Internal method to get the next item in the series following `curr`
    /// from stored data.
    /// 
    fn get_next(&self, curr: &(T, usize)) -> Option<(T, usize)> {
        if let Some(data) = self.series.as_ref().unwrap().get(curr.1 + 1) {
            Some((data.clone(), curr.1 + 1))
        } else { None }
    }
    /// Internal method to push an item onto the series.
    /// 
    fn get(&self, index: usize) -> &T {
        if let Some(series) = &self.series {
            &series[index]
        } else { panic!("No series data"); }
    }
    /// Internal method to push an item onto the series in stored data.
    /// 
    fn push(&mut self, x: T) {
        if let Some(series) = &mut self.series {
            series.push(x);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let series = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                      1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut brent = BrentOnline::new();

        for (i, &x) in series.iter().cycle().enumerate() {
            if brent.add(x).is_some() {
                println!("Cycle found in {} loops.0", i);
                break;
            }
        }
        assert_eq!(brent.get_cycle(), Some((10, 0)));
    }

    #[test]
    fn test_2() {
        let series = [1, 2, 1, 3, 7, 6, 7, 8, 9, 10,
                      1, 2, 3, 4, 5, 2, 7, 7, 9, 10];
        let mut brent = BrentOnline::new();

        for (i, &x) in series.iter().cycle().enumerate() {
            if brent.add(x).is_some() {
                println!("Cycle found in {} loops.0", i);
                break;
            }
        }
        assert_eq!(brent.get_cycle(), Some((6, 15)));
    }
}