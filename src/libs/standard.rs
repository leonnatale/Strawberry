use crate::{error::StrawberryError, parser::StrawberryValue};
use rand::Rng;

pub fn strawberry(args: Vec<StrawberryValue>) -> Result<StrawberryValue, StrawberryError> {
    let mut string_to_print = Vec::new();
    for arg in args {
        match arg {
            StrawberryValue::String(string) => string_to_print.push(string),
            StrawberryValue::Number(number) => string_to_print.push(number.to_string()),
            StrawberryValue::NativeFunction(name, _) => string_to_print.push(format!("(Native Function: {})", name)),
            StrawberryValue::Function(name, _,_) => string_to_print.push(format!("(Function: {})", name)),
            StrawberryValue::Empty => string_to_print.push("(Empty)".into())
        };
    }
    println!("{}", string_to_print.join(" "));
    Ok(StrawberryValue::Empty)
}

pub fn fields_forever() -> String {
    let lyrics = [
        "Let me take you down",
        "'Cause I'm going to strawberry fields",
        "Nothing is real",
        "And nothing to get hung about",
        "Strawberry fields forever",
        "Living is easy with eyes closed",
        "Misunderstanding all you see",
        "It's getting hard to be someone, but it all works out",
        "It doesn't matter much to me"
    ];

    let random_index = rand::thread_rng().gen_range(0..lyrics.len());
    lyrics[random_index].to_string()
}

pub fn beatle() -> String {
    let lyrics = [
        "Paul McCartney",
        "John Lennon",
        "George Harrison",
        "Ringo Starr"
    ];

    let random_index = rand::thread_rng().gen_range(0..lyrics.len());
    lyrics[random_index].to_string()
}