use clap::{App, AppSettings::ArgRequiredElseHelp, Arg, ArgMatches};

pub fn _parse_args() -> ArgMatches {
    let stdin_arg = Arg::new("stdin")
        .help("Read hosts from STDIN")
        .long("stdin");

    App::new("NtHiM")
        .version("0.1.4")
        .author("Binit Ghimire <thebinitghimire@gmail.com>, Captain Nick Lucifer* <naryal2580@gmail.com>")
        .about("Now, the Host is Mine! - Super Fast Sub-domain Takeover Detection!")
        .setting(ArgRequiredElseHelp)
        .args(&[
            Arg::new("file")
                .help("List of Hostnames separated with new line (\\n)!")
                .short('f')
                .long("file")
                .takes_value(true),
            Arg::new("target")
                .help("Hostname with the protocol defined!")
                .short('t')
                .long("target")
                .takes_value(true),
            Arg::new("threads")
                .help("Number of Concurrent Threads! (default: 10)")
                .short('c')
                .long("threads")
                .takes_value(true),
            Arg::new("timeout")
                .help("Timeout for connections (in seconds)! (default: 5)")
                .short('s')
                .long("timeout")
                .takes_value(true),
            Arg::new("verbose")
                .help("Enable Verbose Mode!")
                .short('v')
                .long("verbose")
                .takes_value(false),
            Arg::new("output")
                .help("Write output to file!")
                .short('o')
                .long("output")
                .takes_value(true),
            Arg::new("update")
                .help("Update signature cache!")
                .short('u')
                .long("update")
                .takes_value(false),
            stdin_arg,
        ])
        .get_matches()
}
