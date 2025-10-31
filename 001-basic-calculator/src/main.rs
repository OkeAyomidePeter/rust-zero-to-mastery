use std::io;// Imports the standard library's io module into scope so you can call io::stdin() instead of std::io::stdin(). The book shows this pattern for reading from standard input.
fn main() { // Defines the program entry point. main is the function the runtime calls to start your program. Functions are declared with fn in the book.
    println!("Hello there this is a basic calculator application,usage instructions are below");//Calls the println! macro to print a line to standard output with a newline. println! is a formatting macro documented in examples in the book.
    println!("Type in the first number then pick the operand then pick the second number and the program will calculate it");
    println!("First number: ");//here we prompt the user for input
    let mut first_number: String = String::new();//here we create a mutable variable named first_number,mutable in the sense that we want to further manipulate or change it later on;String is simply a growable string,its different from &str(the default manner which strings are stored) we usually deal with,the new() is a sub funtion of the String that specifies that this is a new string.
    //let mut first_number: String = String::new(); creates a new, empty, growable, owned String. mut makes the binding mutable so methods like read_line can append into it. The book explains String::new, String as an owned UTF-8 buffer, and differences vs &str. 
    io::stdin().read_line(&mut first_number).expect("");//remember we imported the std library and called the io function from it,stdin() is a sub function of io,then we read the last input of the user using read_line() where we referenced the mutable variable first_number using & ,the expect() is for catching unexpected errors.
    //io::stdin() returns a handle to standard input. .read_line(&mut first_number) appends what the user types (including the newline) into the String you passed by mutable reference. read_line returns a Result; expect(...) unwraps Ok or panics with the message on Err. The guessing-game tutorial uses this exact pattern and explains Result handling with expect.
    let first_number:u64 = first_number.trim().parse().expect("");//since the first_number is mutable we can do this first_number = first_number and other stuff,the trim() is for removing the trailing whitespace befre and after the string,parse() is for converting data types though it can be complex because you have to specify what you are converting to from the onset of the assignment ie let x "":i32 this is where we specify that we are parsing to" = other stuff
    //first_number.trim() returns a &str slice with leading/trailing whitespace removed. .parse() attempts to parse that &str into the target type, here u64, inferred from the left side. .parse() returns a Result; expect unwraps or panics. The book shows trimming and parsing user input into numeric types and explains that you must specify the target numeric type. See Data Types and the guessing game example.
    println!("Type in the operand\n we support only these operands [+ for addition] [- for subtraction] [* for multiplication] [/ for division] [% for remainder]");// \n is a newline escape inside the string literal.
    let mut operand: String = String::new();
    io::stdin().read_line(&mut operand).expect("");
    let operand:String = operand.trim().parse().expect("");//operand.trim() yields a &str without whitespace. .parse() converts the trimmed &str into a String (the target type). This is valid because String implements FromStr, so parse::<String>() returns a Result<String, _>. expect unwraps the result or panics. The book explains parse() and type inference for parse.
    println!("Second number: ");
    let mut second_number: String = String::new();
    io::stdin().read_line(&mut second_number).expect("");
    let second_number:u64 = second_number.trim().parse().expect("");//Trims whitespace, parses the &str into u64, and unwraps the Result with expect. Same pattern as for first_number. The book shows parsing numeric input and explains numeric types such as u64.
    println!("{first_number} {operand} {second_number}");
    let result = calculate(first_number, operand, second_number);//Calls the calculate function passing the three values. The call binds the returned u64 into result. Function calling and passing ownership/moving values is shown in the book’s function chapter.
    println!("Result: {}", result)
}//Closes main. End of function scope. Functions and scope are covered in the functions and ownership chapters.

fn calculate(first_number:u64,operand:String,second_number:u64) -> u64  { // Declares a function named calculate that takes two u64s and an owned String, and returns a u64. The -> u64 syntax is how Rust declares a function return type. The book explains function signatures and return types.
if operand == "+" { //Begins an if expression that compares the owned String operand to the string slice literal "+"". String implements PartialEq<&str> so direct equality with a &str literal is supported. The book covers control flow with if and equality semantics in examples.
   first_number + second_number //This is the last expression in the if branch. Because it has no semicolon it is an expression value and becomes the value produced by the if arm. In function bodies that final expression is returned implicitly when the if expression is used as the tail expression. The functions chapter explains that expressions without a semicolon return values.
} else if operand == "-" { // Another if branch. else if chains are allowed. All arms of the if/else if/else must produce values of the same type because the whole if expression yields a single type. The book explains this requirement for expressions.
    first_number - second_number // Subtracts and yields the result as the if arm value. No semicolon so it returns the numeric value
} else if operand == "*" { // Third branch for multiplication. Same behavior as prior branches.
    first_number * second_number //Multiplication expression returned by this branch.
} else if operand == "/" {//fourth branch for division. Same behavior as prior branches.
  first_number / second_number //division expression returned by this branch.
} else if operand == "%" { // Remainder operator branch.
   first_number % second_number//Returns the remainder expression as the branch value.
} else if operand == "/" && second_number == 0{ // over here i handled zero division error in a very lazy manner by throwing an arbitrary 0 back to the user if they make the divisor a 0. i used the and (&&) logical operator and what it does is that the if condition is said to be true only when both conditions sre true.
   0
}else if operand == "%" && second_number == 0 { // over here i handled zero division error in a very lazy manner by throwing an arbitrary 0 back to the user if they make the divisor a 0. 
   0
}
else { // Fallback else branch for unsupported operands. if as an expression still requires a consistent return type from every arm. You return 0 here to keep the branches all returning u64. The book explains that if is an expression and arms must yield the same type.
    0 //The fallback numeric value returned by the else branch. Because it is the final expression of the function body it becomes the function return when none of the prior arms matched. 
}
} // Closes the if expression and the function body. The final expression of the block is returned implicitly if not followed by a semicolon. The functions chapter describes implicit returns via tail expressions. 