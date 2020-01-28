use std::env::args;
use std::fs::File;
use std::io::{Result, Write};

use askama::Template;

struct TypeAndDefault<'a> {
    _type: &'a str,
    default: &'a str,
}

#[derive(Template)]
#[template(path = "template.askama.wurst.txt")]
struct WurstMaybePrimitives<'a> {
    type_and_defaults: &'a [TypeAndDefault<'a>],
}

fn main() -> Result<()> {
    let mut out_file = File::create(args().nth(1).unwrap_or_else(|| "out.wurst".into()))?;

    let template = &WurstMaybePrimitives {
        type_and_defaults: &[
            TypeAndDefault {
                _type: "int",
                default: "0",
            },
            TypeAndDefault {
                _type: "real",
                default: "0.",
            },
            TypeAndDefault {
                _type: "bool",
                default: "false",
            },
            TypeAndDefault {
                _type: "string",
                default: "null",
            },
            TypeAndDefault {
                _type: "unit",
                default: "null",
            },
            TypeAndDefault {
                _type: "player",
                default: "null",
            },
            TypeAndDefault {
                _type: "vec2",
                default: "ZERO2",
            },
            TypeAndDefault {
                _type: "vec3",
                default: "ZERO3",
            },
            TypeAndDefault {
                _type: "angle",
                default: "angle(0.)",
            },
        ][..],
    };

    out_file.write_all(
        template
            .render()
            .expect("Failed to render template")
            .as_bytes(),
    )?;

    Ok(())
}
