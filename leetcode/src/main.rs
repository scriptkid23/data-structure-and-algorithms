mod sun;
mod richest_customer_wealth;

fn main() {
   let x = richest_customer_wealth::maximum_wealth(vec![vec![1,2,3], vec![4,5,6]]);
   print!("{}",x);
}
