fn main() {
    // Check if the commit shows up on github
    let mut x: i32 = 14;
    print!("{x}");
    while x !=1{
        if x%2==0{
            x=x/2;

        }else{
            x=3*x+1;

        }
        print!(" -> {x}");
    }
    println!();
}
