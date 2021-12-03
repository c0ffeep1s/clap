//! Usage example of `arg_enum`
//!
//! All the variants of the enum and the enum itself support `rename_all`

use clap::{ArgEnum, Parser};

#[derive(ArgEnum, Debug, PartialEq, Clone)]
enum ArgChoice {
    /// Descriptions are supported as doc-comment
    Foo,
    // Renames are supported
    #[clap(name = "b-a-r")]
    Bar,
    // Aliases are supported
    #[clap(alias = "b", alias = "z")]
    Baz,
    // Hiding variants from help and completion is supported
    #[clap(hidden = true)]
    Hidden,
}

#[derive(Parser, PartialEq, Debug)]
struct Opt {
    #[clap(arg_enum)]
    arg: ArgChoice,
}

fn main() {
    let opt = Opt::parse();
    println!("{:#?}", opt);
}
