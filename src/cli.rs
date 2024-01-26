use clap::{Command, Arg};

pub fn parse_args() -> clap::ArgMatches {
    Command::new("xhs_downloader")
        .version("0.1")
        .author("Bojin Li")
        .about("Downloads stuff from XiaoHongShu")
        .arg(
            Arg::new("URL")
                .help("Sets the input URL to use")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("type")
                .long("type")
                .value_name("TYPE")
                .help("Sets the type of the download (png, jpg, webp, original)")
        )
        .arg(
            Arg::new("output")
                .long("output")
                .value_name("OUTPUT")
                .help("Sets the output file or directory")
        )
        .subcommand(
            Command::new("webapi")
                .about("Runs as a web API")
                .arg(
                    Arg::new("BIND")
                        .help("Sets the bind address for the web API")
                        .required(false)
                        .index(1),
                ),
        )
        .get_matches()
}
