
# To access postgres inside this one of my windows machine 
# password: 1234

# this doesn't work
# psql "sslmode=disable dbname=postgres user=postgres hostaddr=http://localhost:5432"
psql "postgres://postgres:1234@localhost:5432/postgres"

psql -U postgres

# create rust project 
cargo new --lib diesel_demo
cd diesel_demo

# Install the diesel utility CLI
echo $PQ_LIB_DIR
export PQ_LIB_DIR="C:\Program Files\PostgreSQL\15\lib"
cargo install diesel_cli --no-default-features --features postgres

# Check if diesel is installed
diesel

echo DATABASE_URL=postgres://postgres:1234@localhost/diesel_demo > .env
diesel setup

# Check if database is created
psql "postgres://postgres:1234@localhost:5432/postgres"
\l
\q

# Create the sql files that will be used to create and remove the tables create_posts
diesel migration generate create_posts

# Apply migrations
diesel migration run
diesel migration redo

# How to drop database
psql "postgres://postgres:1234@localhost:5432/postgres"
DROP DATABASE diesel_demo;
\q

export PQ_LIB_DIR="C:\Program Files\PostgreSQL\15\lib"
cargo run --bin show_posts










