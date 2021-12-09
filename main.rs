fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
    
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}