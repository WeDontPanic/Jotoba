# Jotoba
A free online, selfhostable, multilang japanese dictionary.

# Requirements
- [Jmdict.xml](https://www.edrdg.org/wiki/index.php/JMdict-EDICT_Dictionary_Project)
- PostgresDB
- [Diesel](https://github.com/diesel-rs/diesel) with postgres feature

# Installation
1. Setup a postgres DB
2. Customize and run `echo DATABASE_URL=postgres://username:password@localhost/jotoba > .env` 
3. Run `diesel setup && diesel migration run`
4. Compile it: `cargo build --release`
<br>
The binary will be located in ./target/release/jotoba
Joto-kun (including all of his variants) is licensed under [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/).
