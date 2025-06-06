#!/bin/bash

# Function to pull the MariaDB image
pull_mariadb_image() {
    podman pull docker.io/library/mariadb:latest
}

# Function to run the MariaDB container
run_mariadb_container() {
    CURRENT_DIR=$(pwd)

    # Read parameters from runtime.json
    ROOT_PASSWORD=$(jq -r '.mariadb.rootPassword' "$CURRENT_DIR/runtime.json")
    DATABASE=$(jq -r '.mariadb.database' "$CURRENT_DIR/runtime.json")
    HOST_PORT=$(jq -r '.mariadb.hostPort' "$CURRENT_DIR/runtime.json")
    CONTAINER_PORT=$(jq -r '.mariadb.containerPort' "$CURRENT_DIR/runtime.json")

    # Create the .runtime and .runtime/mariadb directories
    mkdir -p "$CURRENT_DIR/.runtime/mariadb"

    podman run -d \
        --name sqlsmith-rs_mariadb \
        -e MYSQL_ROOT_PASSWORD="$ROOT_PASSWORD" \
        -e MYSQL_DATABASE="$DATABASE" \
        -v "$CURRENT_DIR/.runtime/mariadb:/var/lib/mysql" \
        -p "$HOST_PORT:$CONTAINER_PORT" \
        --replace \
        docker.io/library/mariadb:latest

    echo "MariaDB container started with volume mount. Data is saved in $CURRENT_DIR/.runtime/mariadb. You can now connect to it at localhost:$HOST_PORT with root password '$ROOT_PASSWORD' and database '$DATABASE'."
}

# Function to show container status
show_container_status() {
    podman ps -a --filter "name=sqlsmith-rs_mariadb"
}

# Function to show help
show_help() {
    echo "Usage: $0 [OPTIONS]"
    echo "Options:"
    echo "  -p  Pull the MariaDB container image."
    echo "  -r  Run the MariaDB container."
    echo "  -l  Show the current status of the MariaDB container."
    echo "  -h  Show this help message."
}

# If no arguments are provided, show help
if [ $# -eq 0 ]; then
    show_help
    exit 0
fi

# Parse command line arguments
while getopts "prlh" opt; do
    case $opt in
        p)
            pull_mariadb_image
            ;;
        r)
            run_mariadb_container
            ;;
        l)
            show_container_status
            ;;
        h)
            show_help
            ;;
        \?)
            echo "Invalid option: -$OPTARG" >&2
            show_help
            exit 1
            ;;
    esac
done