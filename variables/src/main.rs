fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x after shadowing is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces {}", spaces);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    //tuple usage
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("one {}, five_hundred {}, six_point_four {}", one, five_hundred, six_point_four);

    //array
   let a: [i32; 5] = [1, 2, 3, 4, 5];

   println!("array a0 {}", a[0]);

   let z = some_function(2,3);
   println!("The value of z is: {}", z);

   even_odd(3);
   even_odd(2);

   for_loop();

   let z = fibonacci(10);
   println!("fib of z(10) is {}", z);
}

fn some_function(x: i32, y: i32) -> i32 {
    let z = {
        let s = 3;
        s + 1 + x + y
	
    };
    z
}

fn even_odd(x: i32) {
	if x % 2 == 0 {
		println!("{} is even", x)
	} else if x % 2 != 0 {
		println!("{} is odd", x)
	} else {
		println!("some number")
	}
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        print!("the value is: {} ", element);
    }
    println!("");

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("end");

}

fn fibonacci(num: i32) -> i32 {
	if num == 0 {
	   return 0
        } else if num == 1 {
           return 1 
	} else {
	   return fibonacci(num-1) + fibonacci(num-2)
	}
}
