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
    
}