use clap::parser::ValueSource;
use log::info;

use crate::apis::{ dumper, studio_version };

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    colog::init();

    let arg_matches = clap::Command
        ::new("dump-roblox-api")
        .about("Dumps all Classes and Enums into a human-readable format.")
        .version("1.00")
        .author("@bob448 on github")
        .arg(
            clap::Arg
                ::new("studio-version")
                .long("studio-version")
                .short('v')
                .help("use this studio version instead of the latest version")
                .value_name("version")
                .num_args(1)
        )
        .arg(
            clap::Arg
                ::new("table")
                .long("table-format")
                .short('t')
                .help("format the result as Luau tables")
                .num_args(0)
        )
        .arg(
            clap::Arg
                ::new("hide_non_browseable")
                .long("hide-non-browseable")
                .short('b')
                .help("Hide non-browseable classes (table formatting)")
                .long_help(
                    "Hide non-browseable classes\nOnly applies when table formatting is enabled."
                )
                .num_args(0)
        )
        .arg(
            clap::Arg
                ::new("hide_non_creatable")
                .long("hide-non-creatable")
                .short('c')
                .help("Hide non-creatable classes (table formatting)")
                .long_help(
                    "Hide non-creatable classes\nOnly applies when table formatting is enabled."
                )
                .num_args(0)
        )
        .get_matches();

    start_dumping(
        arg_matches.get_one("studio-version"),
        arg_matches.value_source("table").unwrap() != ValueSource::DefaultValue,
        arg_matches.value_source("hide_non_creatable").unwrap() != ValueSource::DefaultValue,
        arg_matches.value_source("hide_non_browseable").unwrap() != ValueSource::DefaultValue
    ).await
}

async fn start_dumping(
    studio_version: Option<&String>,
    table_format: bool,
    hide_non_creatable: bool,
    hide_non_browseable: bool
) -> color_eyre::Result<()> {
    let studio_version = match studio_version {
        Some(v) => {
            info!("using version \"{}\"", &v);
            v.clone()
        }
        None => {
            let v = studio_version::get_studio_version().await?;
            info!("found version \"{}\"", v);
            v
        }
    };

    let dump = dumper::try_dump(studio_version).await?;

    if table_format {
        info!("printing out dump as luau table\n");

        println!(
            "Class Names:\n{}\n",
            dump.class_names_into_luau_table(hide_non_creatable, hide_non_browseable)
        );
        println!(
            "Classes:\n{}\n",
            dump.classes_into_luau_table(hide_non_creatable, hide_non_browseable)
        );
        println!("Enums:\n{}", dump.enums_into_luau_table());
    } else {
        dbg!(dump);
    }

    Ok(())
}

mod apis;
