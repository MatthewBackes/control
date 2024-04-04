fn main() {
    let num = 7;

    if num < 5 {
        println!("Condition is true.");
    } else {
        println!("Condition is false.");
    }

    let cond = true;
    let num_cond = if cond {5} else {6};
    println!("Value of numCond is {num_cond}");

    let mut counter = 0;

    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result from loop is {result}");

    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("Final count from nested loops = {count}");

    let mut whiletest = 3;

    while whiletest !=0 {
        println!("While : {whiletest}");
        whiletest -= 1;
    }

    println!("Done with while loop.");

    let array = [10, 20, 30, 40, 50];

    for element in array {
        println!("Value of element in array is {element}");
    }
    println!("Done with for loop.");


}