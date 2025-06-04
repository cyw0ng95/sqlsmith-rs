#!/bin/bash

# Function to pull the MariaDB image
pull_mariadb_image() {
    podman pull docker.io/library/mariadb:latest
}

# Function to run the MariaDB container
run_mariadb_container() {
    CURRENT_DIR=$(pwd)

    # Create the .runtime and .runtime/mariadb directories
    mkdir -p "$CURRENT_DIR/.runtime/mariadb"

    podman run -d \
        --name sqlsmith-rs_mariadb \
        -e MYSQL_ROOT_PASSWORD=rootpassword \
        -e MYSQL_DATABASE=fuzzerdb \
        -v "$CURRENT_DIR/.runtime/mariadb:/var/lib/mysql" \
        -p 13306:3306 \
        docker.io/library/mariadb:latest

    echo "MariaDB container started with volume mount. Data is saved in $CURRENT_DIR/.runtime/mariadb. You can now connect to it at localhost:3306 with root password 'rootpassword' and database 'fuzzerdb'."
}

# Function to show help
show_help() {
    echo "Usage: $0 [OPTIONS]"
    echo "Options:"
    echo "  -p  Pull the MariaDB container image."
    echo "  -r  Run the MariaDB container."
    echo "  -h  Show this help message."
}

# If no arguments are provided, show help
if [ $# -eq 0 ]; then
    show_help
    exit 0
fi

# Parse command line arguments
while getopts "prh" opt; do
    case $opt in
        p)
            pull_mariadb_image
            ;;
        r)
            run_mariadb_container
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