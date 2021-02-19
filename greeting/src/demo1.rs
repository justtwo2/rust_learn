use std::f64::consts::PI;
use std::fs::File;


fn main(){
    let mut x = 5 ;
    let y  = x ;
    x = x + 1;
    println!("x 为 {0} ,y 为 {1}", x,y);

    let str1 = String::from("hello"); 
    println!("{}",str1);
    let str2  = str1.clone() ;
    println!("{}",str2);
    //str1 = String::from("hello abc");
    // println!("{}",str1);
    
    println!("x 为 {0} ,y 为 {1}", str1,str2);
    println!("PI 为 {}", PI);
} 