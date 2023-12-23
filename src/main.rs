fn main(){
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    //shadowing
    let y=5;

    println!("The value of y is: {}", y);

    let y = 7;
    println!("The changed value of y after shadowing  is: {}", y);

    let y = "how are you";
    println!("The changed value of y after shadowing  is: {}", y);


    const SUB_COUNT: u32 = 100_000;
    println!("The value of SUB_COUNT is: {}", SUB_COUNT);
}