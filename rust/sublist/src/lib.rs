#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
//thank you to the community solution from alireza4050 -- dont think 
//i would have known how array windows worked without it.
//found here https://exercism.org/tracks/rust/exercises/sublist/solutions/alireza4050
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let a = _first_list;
    let b = _second_list;
    match(a.len(), b.len()){
        //these first 3 are obvious -- matching lengths
        (0,0)        => Comparison::Equal,
        (0,_)        => Comparison::Sublist,
        (_,0)        => Comparison::Superlist,
        //these next 2 are a little special -- functional style workings here 
        //windows gives you an iterator that goes over the array in groupings of size n -- [1,2,3,4].windows(2)
        //gives you [1,2], [2,3], [3,4], for example. any is just a search based on a predicate. 
        //runtime wise, this takes however long it takes to make the predicate + at least O(n) 
        //since the window searching will always be linear
        (m,n) if m>n => if a.windows(n).any(|v| v==b) {Comparison::Superlist} else {Comparison::Unequal},
        (m,n) if m<n => if b.windows(m).any(|v| v==a) {Comparison::Sublist}   else {Comparison::Unequal},
        (_,_)        => if a==b {Comparison::Equal} else {Comparison::Unequal},
    }
}
