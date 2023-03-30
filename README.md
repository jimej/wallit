
# wallit: Protecting passwords and other secrets

cargo run -- -n Erin -c 2

cargo run -- -V

cargo run -- add -k x -v y
cargo run -- -d add -k x -v y

add .env, then diesel setup
diesel migration generate ..
sqlite3 -> .help -> .open crud.db -> select * from companies -> .quit
removed Nullable in schema.rs for companies

add/modify/generate/delete (not allowed?) -> add/rotate (including modify - mode being shown as there was mistake)/show (last 3)
how to protect sqlite database file
open source, take PR -> use ideas to create product

cargo run -- add -t companies -c zzz540jty -v https://xz31.com