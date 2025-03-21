use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::process::exit;
use clap::Parser;
use bcrypt::verify;
use figlet_rs::FIGfont;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256, Digest};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use ripemd::{Ripemd128, Ripemd160, Ripemd256, Ripemd320};
use scrypt::{scrypt, Params};
use argon2::{Argon2, PasswordHash, PasswordVerifier};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short = 'H', long, conflicts_with = "hash_file")]
    hash: Option<String>,

    #[clap(long = "hash-file", conflicts_with = "hash")]
    hash_file: Option<String>,

    #[clap(short, long)]
    wordlist: String,
}

fn handle_scrypt(password: &str, hash: &str) -> bool {
    let parts: Vec<&str> = hash.split('$').filter(|s| !s.is_empty()).collect();
    if parts.len() != 6 || parts[0] != "s2" {
        eprintln!("Invalid scrypt hash format");
        exit(1);
    }

    let log_n = parts[1].parse::<u8>().unwrap();
    let r = parts[2].parse::<u32>().unwrap();
    let p = parts[3].parse::<u32>().unwrap();
    let salt = parts[4].as_bytes();
    let stored_key = hex::decode(parts[5]).unwrap();

    // Add the output length as the 4th parameter
    let params = Params::new(log_n, r, p, stored_key.len()).unwrap();
    let mut output = vec![0u8; stored_key.len()];
    scrypt(password.as_bytes(), salt, &params, &mut output).unwrap();

    output == stored_key
}

fn decrypt_hash(hash: &str, hash_type: &str, password_list: &str) {
    let mut attempts = 1;
    println!("Attempting to crack {}! \n", hash);

    let password_list = File::open(password_list).unwrap_or_else(|_| {
        eprintln!("Failed to open wordlist file");
        exit(1);
    });
    let reader = BufReader::new(password_list);

    for line in reader.lines() {
        let line = line.unwrap_or_else(|_| {
            eprintln!("Failed to read line from wordlist");
            exit(1);
        });
        let password = line.trim();

        match hash_type {
            "BCrypt" => {
                if verify(password, hash).unwrap_or(false) {
                    println!("BCrypt hash found after {} attempts! Password: {}", attempts, password);
                    exit(0);
                }
            },
            "scrypt" => {
                if handle_scrypt(password, hash) {
                    println!("scrypt hash found after {} attempts! Password: {}", attempts, password);
                    exit(0);
                }
            },
            "Argon2" => {
                let parsed_hash = PasswordHash::new(hash).unwrap();
                if Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok() {
                    println!("Argon2 hash found after {} attempts! Password: {}", attempts, password);
                    exit(0);
                }
            },
            _ => {
                let password_bytes = password.as_bytes();
                let password_hash = match hash_type {
                    // MD Family
                    "MD5" => format!("{:x}", Md5::digest(password_bytes)),
                    
                    // SHA-1
                    "SHA-1" => format!("{:x}", Sha1::digest(password_bytes)),
                    
                    // SHA-2 Family
                    "SHA-224" => format!("{:x}", Sha224::digest(password_bytes)),
                    "SHA-256" => format!("{:x}", Sha256::digest(password_bytes)),
                    "SHA-384" => format!("{:x}", Sha384::digest(password_bytes)),
                    "SHA-512" => format!("{:x}", Sha512::digest(password_bytes)),
                    "SHA-512/224" => format!("{:x}", Sha512_224::digest(password_bytes)),
                    "SHA-512/256" => format!("{:x}", Sha512_256::digest(password_bytes)),
                    
                    // SHA-3 Family
                    "SHA3-224" => format!("{:x}", Sha3_224::digest(password_bytes)),
                    "SHA3-256" => format!("{:x}", Sha3_256::digest(password_bytes)),
                    "SHA3-384" => format!("{:x}", Sha3_384::digest(password_bytes)),
                    "SHA3-512" => format!("{:x}", Sha3_512::digest(password_bytes)),
                    
                    // RIPEMD Family
                    "RIPEMD-128" => format!("{:x}", Ripemd128::digest(password_bytes)),
                    "RIPEMD-160" => format!("{:x}", Ripemd160::digest(password_bytes)),
                    "RIPEMD-256" => format!("{:x}", Ripemd256::digest(password_bytes)),
                    "RIPEMD-320" => format!("{:x}", Ripemd320::digest(password_bytes)),
                    
                    _ => {
                        eprintln!("Unsupported hash type: {}", hash_type);
                        exit(1);
                    }
                };

                if password_hash == hash {
                    println!("{} hash found after {} attempts! Password: {}",
                             hash_type, attempts, password);
                    exit(0);
                }
            }
        }
        attempts += 1;
    }
}

fn main() {
    let standard_font = FIGfont::standard().unwrap();
    if let Some(figure) = standard_font.convert("CrabCrack") {
        println!("{}", figure);
    }

    let args = Args::parse();

    let hash = match (args.hash, args.hash_file) {
        (Some(hash), None) => hash,
        (None, Some(hash_file)) => {
            let mut file = File::open(&hash_file).unwrap_or_else(|_| {
                eprintln!("Failed to open hash file: {}", hash_file);
                exit(1);
            });
            let mut hash = String::new();
            file.read_to_string(&mut hash).unwrap_or_else(|_| {
                eprintln!("Failed to read hash from file: {}", hash_file);
                exit(1);
            });
            hash.trim().to_string()
        }
        _ => {
            eprintln!("Either --hash or --hash-file must be provided, but not both");
            exit(1);
        }
    };

    let hash_type = identyhash::identify_hash(&hash);
    decrypt_hash(&hash, &hash_type, &args.wordlist);
}