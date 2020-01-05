build:
    sudo systemctl start postgresql.service
    cargo build
develop:
    cd js && npm run build &
    cd ..
    cargo run