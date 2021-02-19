fn another_function(mut x: i32, mut y: i32) {
    x = x + 1;
    y = y + 1;
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
}

fn main() {
    let mut a = 12;
    let mut b = 24;
    println!("Hello, world!");
    print!("Hello, world!\n");
    println!("a is {0}, b is {1} a again is {0}", a,b); 
    another_function(a,b);
    a = a + 1;
    b = b + 1; 
    another_function(a,b);
    println!("a is {0}, b is {1} a again is {0}", a,b); 
    let number = if a > 0 && b >50 { 1 } else { -1 };
    println!("number 为 {}", number);

    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }

    for i in 0..5{
        println!("{}",i);
    }
    let arr1 = [1,2,3,4,5,6];
    for i in arr1.iter(){
        println!("{}",i);
    }
    loop{
        a = a + 1;
        if a > 20{
            break;
        }
        println!("{}",a)
    }

}
