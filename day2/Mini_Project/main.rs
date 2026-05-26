// Simple Calculator

/*
 Create two variables: num1 = 20 and num2 = 5
 Create a third variable: result = num1 + num2
 Print: "20 + 5 = 25" (using your variables)
 Try subtraction (-), multiplication (*), and division (/) too
*/

fn main(){
    let num1 = 20;
    let num2 = 5;
    let result_add = num1 + num2;
    let result_subtract = num1 - num2;
    let result_multiply = num1 * num2;
    let result_divide = num1 / num2;

    println!("{} + {} = {}", num1,num2,result_add);
    println!("{} 1 {} = {}", num1,num2,result_subtract);
    println!("{} * {} = {}", num1,num2,result_multiply);
    println!("{} / {} = {}", num1,num2,result_divide);
}