extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    

    loop
    {
        println!("Please Enter Following?");
        println!("1=>Rock 2=>Scissors 3=>Paper");

        let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line");

        let input: i32 = input.trim().parse()
            .expect("Please type a number!");
        if input > 3
        {
            println!("Please Select One Of The Options");
            continue;
        }
        else if input < 0
        {
            println!("Please Select One Of The Options");
            continue;
        }

        let mut rng = rand::thread_rng();
        let comp = rng.gen_range(1,4);
        match input
        {
            1=>println!("You have Selected Rock"),
            2=>println!("You have Selected Scissors"),
            3=>println!("You have Selected Paper"),
            _=>println!("")
        }

        match comp
        {
            1=>println!("Computer have Selected Rock"),
            2=>println!("Computer have Selected Scissors"),
            3=>println!("Computer have Selected Paper"),
            _=>println!("")
        }

        if comp == input
        {
            println!("Congrats You Have Won");
            break;
        }
        else
        {
            println!("Alass You Have Lost");
        }

        let mut check = 0;
        
        loop
        {
            println!("Do You Want To Try Again (y/n)?");
            let mut yes_no = String::new();
                io::stdin().read_line(&mut yes_no)
                    .expect("Failed to read line");

            let yes_no: char = yes_no.trim().parse()
                .expect("Please type a number!");
         
            if yes_no == 'y' || yes_no == 'Y'
            {
                check=1;
                break;
            }
            else if yes_no == 'n' || yes_no == 'N'
            {
                check=2;
                break;
            }

        }
        if check == 1
        {
            continue;
        }
        else if check == 2
        {
            break;
        }

    }
}
