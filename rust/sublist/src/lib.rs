#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
//in prog on this one, taking a break
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let mut count = 0;
    if _first_list.len()==0 && _second_list.len()==0{
        return Comparison::Equal;
    }else if _first_list.len()==0{
        return Comparison::Sublist;
    }else if _second_list.len()==0{
        return Comparison::Superlist;
    }
    for (x,y) in _first_list.iter().zip(_second_list.iter()){
        if x==y{
            count += 1;
            if count == _first_list.len() || count == _second_list.len(){
                if _first_list.len() < _second_list.len(){
                    return Comparison::Sublist;
                }else if _first_list.len() > _second_list.len(){
                    return Comparison::Superlist;
                }else{
                    return Comparison::Equal;
                }
            }
        }else{
            count = 0;
        }
    }
    return Comparison::Unequal;
}
