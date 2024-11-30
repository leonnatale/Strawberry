use crate::{error::StrawberryError, parser::{StrawberryParser, StrawberryValue}};
use rand::Rng;

pub fn strawberry(args: Vec<StrawberryValue>, _: &mut StrawberryParser) -> Result<StrawberryValue, StrawberryError> {
    let mut string_to_print = Vec::new();
    for arg in args {
        match arg {
            StrawberryValue::String(string) => string_to_print.push(string),
            StrawberryValue::Number(number) => string_to_print.push(number.to_string()),
            StrawberryValue::NativeFunction(name, _) => string_to_print.push(format!("(Native Function: {})", name)),
            StrawberryValue::Function(name, _,_) => string_to_print.push(format!("(Function: {})", name)),
            StrawberryValue::Boolean(boolean) => string_to_print.push(format!("{boolean}")),
            StrawberryValue::Block(_) => string_to_print.push(format!("(Code block)")),
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

pub fn execute_code_block(args: Vec<StrawberryValue>, context: &mut StrawberryParser) -> Result<StrawberryValue, StrawberryError> {
    let arg0 = args.get(0).unwrap();
    let mut result = StrawberryValue::Empty;
    if let StrawberryValue::Block(code) = arg0 {
        result = StrawberryParser::new(code.iter().map(|t| *t.clone()).collect(), context.variables.clone()).run_token_stream()?;
    }
    Ok(result)
}

pub fn if_comparison(mut args: Vec<StrawberryValue>, context: &mut StrawberryParser) -> Result<StrawberryValue, StrawberryError> {
    // Remove o primeiro argumento, que deve ser a condição
    let condition = args.remove(0);

    // Verifica se a condição é um booleano
    if let StrawberryValue::Boolean(boolean) = condition {
        if boolean {
            // Executa o bloco "if" (primeiro bloco no restante dos argumentos)
            if let Some(if_block) = args.get(0) {
                return execute_code_block(vec![if_block.clone()], context);
            }
        } else {
            // Executa o bloco "else" (segundo bloco no restante dos argumentos, se existir)
            if let Some(else_block) = args.get(1) {
                return execute_code_block(vec![else_block.clone()], context);
            }
        }
    } else {
        // Retorna erro se o primeiro argumento não for um booleano
        return Err(StrawberryError::semantic_error(
            "First argument of 'if' must be a boolean",
        ));
    }

    // Retorna vazio se nenhum bloco foi executado
    Ok(StrawberryValue::Empty)
}