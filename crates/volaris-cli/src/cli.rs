use clap::{Arg, Command};

pub mod prompt;

#[allow(clippy::too_many_lines)]
pub fn get_matches() -> clap::ArgMatches {
    let encrypt = Command::new("encrypt")
        .short_flag('e')
        .about("Encrypt a file")
        .arg(
            Arg::new("input")
                .value_name("input")
                .required(true)
                .help("The file to encrypt"),
        )
        .arg(
            Arg::new("output")
                .value_name("output")
                .required(true)
                .help("The output file"),
        )
        .arg(
            Arg::new("keyfile")
                .short('k')
                .long("keyfile")
                .value_name("file")
                .help("Use a keyfile instead of a password"),
        )
        .arg(
            Arg::new("erase")
                .long("erase")
                .value_name("# of passes")
                .require_equals(true)
                .help("Securely erase the input file once complete (default is 1 pass)")
                .default_value("1"),
        )
        .arg(
            Arg::new("hash")
                .short('H')
                .long("hash")
                .help("Return a BLAKE3 hash of the encrypted file"),
        )
        .arg(
            Arg::new("argon")
                .long("argon")
                .help("Use argon2id for password hashing"),
        )
        .arg(
            Arg::new("autogenerate")
                .long("auto")
                .value_name("# of words")
                .require_equals(true)
                .default_value("7")
                .help("Autogenerate a passphrase (default is 7 words)")
                .conflicts_with("keyfile"),
        )
        .arg(
            Arg::new("header")
                .long("header")
                .value_name("file")
                .help("Store the header separately from the file"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Force all actions"),
        )
        .arg(
            Arg::new("aes")
                .long("aes")
                .help("Use AES-256-GCM for encryption"),
        );

    let decrypt = Command::new("decrypt")
        .short_flag('d')
        .about("Decrypt a file")
        .arg(
            Arg::new("input")
                .value_name("input")
                .required(true)
                .help("The file to decrypt"),
        )
        .arg(
            Arg::new("output")
                .value_name("output")
                .required(true)
                .help("The output file"),
        )
        .arg(
            Arg::new("keyfile")
                .short('k')
                .long("keyfile")
                .value_name("file")
                .help("Use a keyfile instead of a password"),
        )
        .arg(
            Arg::new("header")
                .long("header")
                .value_name("file")
                .help("Use a header file that was dumped"),
        )
        .arg(
            Arg::new("erase")
                .long("erase")
                .value_name("# of passes")
                .require_equals(true)
                .help("Securely erase the input file once complete (default is 1 pass)")
                .default_value("1"),
        )
        .arg(
            Arg::new("hash")
                .short('H')
                .long("hash")
                .help("Return a BLAKE3 hash of the encrypted file"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Force all actions"),
        );

    Command::new("volaris")
        .version(clap::crate_version!())
        .author("brxken128 <brxken128@tutanota.com>, greendoescode <green@apolga.com>")
        .about("Secure, fast and modern command-line encryption of files.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(encrypt.clone())
        .subcommand(decrypt.clone())
        .subcommand(
            Command::new("erase")
                .about("Erase a file completely")
                .arg(
                    Arg::new("input")
                        .value_name("input")
                        .required(true)
                        .help("The file to erase"),
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Force all actions"),
                )
                .arg(
                    Arg::new("passes")
                        .long("passes")
                        .value_name("# of passes")
                        .require_equals(true)
                        .help("Specify the number of passes (default is 1)")
                        .default_value("1"),
                ),
        )
        .subcommand(
            Command::new("hash")
                .about("Hash files with BLAKE3")
                .arg(
                    Arg::new("input")
                        .value_name("input")
                        .required(true)
                        .num_args(1..)
                        .help("The file(s) to hash"),
                ),
        )
        .subcommand(
            Command::new("pack")
                .about("Pack and encrypt an entire directory")
                .short_flag('p')
                .arg(
                    Arg::new("input")
                        .value_name("input")
                        .num_args(1..)
                        .required(true)
                        .help("The directory to encrypt"),
                )
                .arg(
                    Arg::new("output")
                        .value_name("output")
                        .required(true)
                        .help("The output file"),
                )
                .arg(
                    Arg::new("erase")
                        .long("erase")
                        .help("Securely erase every file from the source directory, before deleting the directory"),
                )
                .arg(
                    Arg::new("argon")
                        .long("argon")
                        .help("Use argon2id for password hashing"),
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Show a detailed output"),
                )
                .arg(
                    Arg::new("autogenerate")
                        .long("auto")
                        .value_name("# of words")
                        .require_equals(true)
                        .default_value("7")
                        .help("Autogenerate a passphrase (default is 7 words)")
                        .conflicts_with("keyfile"),
                )
                .arg(
                    Arg::new("header")
                        .long("header")
                        .value_name("file")
                        .help("Store the header separately from the file"),
                )
                .arg(
                    Arg::new("zstd")
                        .short('z')
                        .long("zstd")
                        .help("Use ZSTD compression"),
                )
                .arg(
                    Arg::new("recursive")
                        .short('r')
                        .long("recursive")
                        .help("Index files and folders within other folders (index recursively)"),
                )
                .arg(
                    Arg::new("keyfile")
                        .short('k')
                        .long("keyfile")
                        .value_name("file")
                        .help("Use a keyfile instead of a password"),
                )
                .arg(
                    Arg::new("hash")
                        .short('H')
                        .long("hash")
                        .help("Return a BLAKE3 hash of the encrypted file"),
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Force all actions"),
                )
                .arg(
                    Arg::new("aes")
                        .long("aes")
                        .help("Use AES-256-GCM for encryption"),
                ),
        )
        .subcommand(
            Command::new("unpack")
                .short_flag('u')
                .about("Unpack a previously-packed file")
                .arg(
                    Arg::new("input")
                        .value_name("input")
                        .required(true)
                        .help("The file to decrypt"),
                )
                .arg(
                    Arg::new("output")
                        .value_name("output")
                        .required(true)
                        .help("The output file"),
                )
                .arg(
                    Arg::new("keyfile")
                        .short('k')
                        .long("keyfile")
                        .value_name("file")
                        .help("Use a keyfile instead of a password"),
                )
                .arg(
                    Arg::new("header")
                        .long("header")
                        .value_name("file")
                        .help("Use a header file that was dumped"),
                )
                .arg(
                    Arg::new("erase")
                        .long("erase")
                        .value_name("# of passes")
                        .require_equals(true)
                        .help("Securely erase the input file once complete (default is 1 pass)")
                        .default_value("1"),
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Show a detailed output"),
                )
                .arg(
                    Arg::new("hash")
                        .short('H')
                        .long("hash")
                        .help("Return a BLAKE3 hash of the encrypted file"),
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Force all actions"),
                ),
        )
        .get_matches()
}
