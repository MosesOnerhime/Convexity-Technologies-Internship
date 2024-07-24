mod calculate;
use crate::calculate::calculate::add;
use crate::calculate::calculate::subtract;


fn main() {
    let result_add = add(10, 5);
    let result_subtract = subtract(10, 5);

    println!("Addition: {}", result_add);
    println!("Subtraction: {}", result_subtract);
}
