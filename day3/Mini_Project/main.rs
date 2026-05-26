// Day 3 mini project

/* 
 Write a function celsius_to_fahrenheit(c: f64) -> f64 that converts temperature
 Formula: (c * 9.0 / 5.0) + 32.0
 In main, convert these temps and print each result: 0°C, 100°C, 37°C
 Expected output example: 0°C = 32°F 
 */

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main(){
    let temp1 = 0.0;
    let temp2 = 100.0;
    let temp3 = 37.0;
    let result1 = celsius_to_fahrenheit(temp1);
    let result2 = celsius_to_fahrenheit(temp2);
    let result3 = celsius_to_fahrenheit(temp3);
    
    println!("{temp1}°C = {result1}°F");
    println!("{temp2}°C = {result2}°F");
    println!("{temp3}°C = {result3}°F");
}