# CrabCrack

crabcrack is a hash cracking tool written in the Rust Programming Language

<b>Command Examples</b>
```
./crabcrack --hash 098f6bcd4621d373cade4e832627b4f6 --wordlist rockyou.txt
```
```
./crabcrack --hash-file 098f6bcd4621d373cade4e832627b4f6 --wordlist rockyou.txt
```
librarys to add
```
cargo add sha1 sha2 sha3 md5 ripemd scrypt argon2 figlet_rs clap bcrypt
```

supported algorithms

| Algorithm Family    | Specific Algorithms                     | Example Hash                                                                 |
|---------------------|-----------------------------------------|------------------------------------------------------------------------------|
| **MD**              | MD5                                     | `098f6bcd4621d373cade4e832627b4f6`                                          |
| **SHA-1**           | SHA-1                                   | `a94a8fe5ccb19ba61c4c0873d391e987982fbbd3`                                  |
| **SHA-2**           | SHA-224                                 | `90a3ed9e32b2aaf4c61c410eb925426119e1a9dc`                                  |
|                     | SHA-256                                 | `5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8`          |
|                     | SHA-384                                 | `54a59b9f22b0b80880d8427e548b7c23abd873486e1f035dce9cd697e85175035ca6ae8d6` |
|                     | SHA-512                                 | `ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db27ac185f8` |
|                     | SHA-512/224                             | `d59e997f36e8d3e5d6303cfd7c5b4d1a3a399a6e`                                  |
|                     | SHA-512/256                             | `9f91221c0b131b5b8a9b5b7d7d4703e4e5a1a7a3f0f1e1d1c1b1a19181716151`          |
| **SHA-3**           | SHA3-224                                | `dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a3`                  |
|                     | SHA3-256                                | `dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f`          |
|                     | SHA3-384                                | `84f8e4c1a7d02a9e0b931e5f8d6d6d6c1c7d8e3b0c6a9f0e9d8c7b6a5a4b3a2`          |
|                     | SHA3-512                                | `a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123`  |
| **RIPEMD**          | RIPEMD-128                              | `9d56d0e57a0c206ab3a043a4f1d50879`                                          |
|                     | RIPEMD-160                              | `108f07b8382412612c048d07d13f814118445acd`                                  |
|                     | RIPEMD-256                              | `c39865d89a7a716d0b5b3d756729a03b6786d2680ddc3b3d7d7f7d7d6d5d4d3d2`          |
|                     | RIPEMD-320                              | `302b0a28a3f0eb0d5c4f8f9e8d7c6b5a4b3a2a1a0a9a8a7a6a5a4a3a2a1a`              |
| **Password Hashes** | bcrypt                                  | `$2a$12$R9h/cIPz0gi.URNNX3kh2OPST9/PgBkNEquQWy7iFWqba9Uo1zIWy`              |
|                     | scrypt                                  | `$s2$14$8$1$k61j6h8h5m3n4b1v$d8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9` |
|                     | Argon2                                  | `$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaa`  |

**Notes**:
- MD5/SHA-1 marked as insecure (deprecated for security-sensitive use)
- SHA-2/SHA-3 considered secure for general purpose
- bcrypt/scrypt/Argon2 recommended for password storage
- All hashes automatically detected via `identyhash`
