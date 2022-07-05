use std::io;

fn main() {
    

        
        //This section we learn about variable declarations
        let mut x = 5;
        const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
        // using a constant
        println!("The value of 3 hours in seconds is: {}", THREE_HOURS_IN_SECONDS);
        //shadowing a variable
        x = 5;
        let x = x + 1;
        {
            let x = x * 2;
            println!("The value of x in inner scope is: {}",x);
            // here we will see the value of the shadowed variable overwrites the org value
        }
        println!("The value of x is: {}", x);
        // remember shadowing is very differnt form makeing a variable mutable.

        // This section explores datatypes / Compound types
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        // to get indivisual value we use pattern matching
        let (x,y,z) = tup;
        println!("The value of x is: {}",x);
        println!("The value of y is: {}",y);
        println!("The value of z is: {}",z);
        // or you could access it directly using (.) followed by index
        println!("The value of x is: {}" ,tup.0);


        // arrays : in rust they have a fixed length
        let a = [1,2,3,4,5];
        let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
        let a = [3;5];
        let first = a[0];
     //accessing the element of an array out of bounds
        let a2 = [1, 2, 3, 4, 5];
        
        println!("Please enter an array index.");
        
        let mut index = String::new();
        
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
       
        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");
        
        let element = a2[index];
        
        println!(
            "The value of the element at index {} is: {}",
            index, element);

    

}
