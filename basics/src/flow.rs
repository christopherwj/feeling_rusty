
//loops 

fn loop_it_good {

    loop {

        break;  //let's up leave a loop
    }
}

fn match_statements {
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    }
    
    match guess.cmp(&secret_number) { //match condition
        Ordering::Less => println!("Too Small"), //the first arm
        Ordering::Greater => println!("Too Big"), //the second arm
        Ordering::Equal => println!("Correct!")
    }
}