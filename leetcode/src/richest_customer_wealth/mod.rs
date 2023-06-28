pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut max:Vec<i32> = vec![];
    for i in accounts{
        max.push(i.iter().sum()); 
    }
    return max.iter().max().unwrap().clone();
}