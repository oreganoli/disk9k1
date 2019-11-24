build:
    sudo systemctl start postgresql.service
    diesel migration run
    cargo build
develop:
    cd js && npm start &
    cargo run
    killall -9 npm