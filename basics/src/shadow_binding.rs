fn main() {
    //The following is now allowed!
    let shadow_variable = 12345;
    println!("The first value of this variable is: {}",shadow_variable);
    //shadow_variable = "Chris" ;
    println!("The first value of this variable is: {}",shadow_variable);
    //we get compile time error that chris is expected to be int, but contains string
    
    //the following is allowed!
    let shadow_variable = 12345;
    println!("The first value of this variable is: {}",shadow_variable);
    let shadow_variable = "Chris" ;
    println!("The first value of this variable is: {}",shadow_variable);
    //Adding "let" before the variable name allows us to shadow the variable

    //variables have a scope, and are only binded in blocks, 
    let long_lived_variable = 1;
    println!("{}", long_lived_variable);
    {
        println!("{}", long_lived_variable);
        let short_lived_variable = 2;
        println!("{}", short_lived_variable);
    }
    //println!("{}", short_lived_variable); //this will cause a compile time error
    println!("{}", long_lived_variable);
}