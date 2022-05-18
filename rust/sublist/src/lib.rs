#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
//in prog on this one, taking a break
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {

}
