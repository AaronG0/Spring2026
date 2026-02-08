const FROZEN_FARENHEIT :f64 = 32.0;

fn assignment1() {
    fn far_to_cel(f:f64) -> f64{
        let c: f64 = (f - FROZEN_FARENHEIT) * (5.0/9.0);
        c
    }

    fn cel_to_far(c:f64)-> f64{
        let f = c * (9.0/5.0 as f64) + FROZEN_FARENHEIT;
        f
    }
    let mut temp: f64 = 62.0;
    let mut tempinc: f64 = 0.0;
    tempinc = far_to_cel(temp);
    println!("{} degrees farenheit is approximately {} celcius", temp, tempinc);

    let max: f64 = temp + 5.0;
    temp += 1.0;

    while temp <= max{
        println!("{} degrees farenheit is converted to celcius is {}", temp, far_to_cel(temp));
        temp += 1.0;
    }
}

fn assignment2() {
    fn is_even(num: i32) -> bool{
        if num % 2 == 0{
            return true;
        }
        return false
    }
    let numarray = [1, 2, 3, 4, 5, 6, 7, 8, 9, 15];
    let arrsize = 10;

    for num in numarray {
        if num % 3 == 0{
            if num % 5 == 0{
                println!("{} = FizzBuzz", num);
                continue;
            }
            println!("{} = Fizz", num);
            continue;
        }
        if num % 5 == 0{
            println!("{} = Buzz", num);
            continue;
        }
        if is_even(num){
            println!("{} = Even", num);
        }
        else{
            println!("{} = Odd", num);
        }

    }

    let mut sum = 0;
    let mut i = 0;
    while i < arrsize{
        sum += numarray[i];
        i += 1;
    }
    println!("Sum of array = {}", sum);

    i = 0;
    let mut largest = 0;
    loop {
        if numarray[i] > largest{
            largest = numarray[i];
            i += 1;
        }
        if i >= arrsize{
            break;
        }

    }
    println!("Largest number = {}", largest);
}

fn assignment3() {
    let secretnum: i32 = 10;
    fn check_guess(guess: i32, secret: i32) -> i32{
        if guess == secret{
            return 0;
        }
        if guess > secret{
            return 1;
        }
        else{
            return -1;
        }
    }
    let mut guess: i32 = 5;
    let mut attempts = 1;
    loop{
        if check_guess(guess, secretnum) == 0{
            println!("Congradulations, {} was the secret number!", guess);
            break;
        }
        if check_guess(guess, secretnum) == -1{
            println!("Your guess of {} was too low. Try again.", guess);
            guess += 1;
            attempts += 1;
            continue;
        }
        else{
            println!("Your guess of {} was too high. Try again.", guess);
            guess -= 1;
            attempts += 1;
            continue;
        }
    }
    println!("Attemps taken: {}", attempts);
}

fn main() {
    assignment1();
    assignment2();
    assignment3();
}
