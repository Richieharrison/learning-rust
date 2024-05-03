// fn main() {
//     let number = 3;
//     if number > 5 {
//         println!("The number is greater than five")
//     }else{
//         println!("The number is lesser than five")
//     }
// }

fn main (){
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("The value of the number is {number}")
}