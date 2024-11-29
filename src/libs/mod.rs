mod standard;

use crate::parser::{StrawberryParser, StrawberryValue};

pub fn load_standard(parser: &mut StrawberryParser) {
    parser.variables.insert(
        "strawberry".into(),
        StrawberryValue::NativeFunction("Strawberry".into(), standard::strawberry),
    );
    parser.variables.insert(
        "fields_forever".into(),
        StrawberryValue::String(standard::fields_forever()),
    );
    parser.variables.insert(
        "beatle".into(),
        StrawberryValue::String(standard::beatle()),
    );
}