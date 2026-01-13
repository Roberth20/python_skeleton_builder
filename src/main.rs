use clap::{Arg, ArgAction, Command, command};
use python_skeleton::build_skeleton;

fn cmd() -> Command {
    command!()
        .next_line_help(true)
        .arg(
            Arg::new("project")
                .required(true)
                .value_name("PROJECT_NAME")
                .help("Name of the root directory of the project. It mus be Train-Case."),
        )
        .arg(
            Arg::new("package")
                .required(true)
                .value_name("PKG_NAME")
                .help("Name of the package. It must be snake_case."),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("doc")
                .long("doc")
                .action(ArgAction::SetTrue)
                .help("If present, create a directory `docs` for documentation of the package."),
        )
}

fn main() {
    let matches = cmd().get_matches();
    let result = build_skeleton(
        matches.get_one::<String>("project").unwrap().to_string(),
        matches.get_one::<String>("package").unwrap().to_string(),
        matches.get_flag("verbose"),
        matches.get_flag("doc"),
    );
    match result {
        Ok(_) => println!("Ypur project is ready to work!"),
        Err(_) => println!("Ops, check your inputs and try again."),
    };
}

#[test]
fn verify_app() {
    cmd().debug_assert();
}
