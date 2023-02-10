#![allow(unused)]

fn main() {
    {
        let some_option_value: Option<i32> = None;

        //refutable pattern - None not covered

        // let Some(x) = some_option_value;
    }
    {
        let some_option_value: Option<i32> = None;
        if let Some(x) = some_option_value {
            println!("{}", x);
        }
    }

    {
        // irrefutable pattern - allways - match the compiler will give a warning
        if let x = 5 {
            println!("{}", x);
        };
    }
}
