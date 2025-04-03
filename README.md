# MySQL Backup Tool

This is a simple Rust tool designed to create backups of MySQL databases. The program lists all databases available on a MySQL server and generates an individual `.sql` file for each in a backup directory, excluding system databases (`information_schema`, `performance_schema`, `mysql`, and `sys`).

## Features

- Creates individual backups for each database in `.sql` format.
- Allows specifying a custom port via the `--port` argument.
- Uses the `root` MySQL user by default (configurable in the code).
- Stores backups in `/var/backups/mysql/` (creates the directory if it doesn’t exist).
- Automatically excludes system databases.

## Requirements

- **Rust**: You need the Rust compiler (`cargo`) installed.
- **MySQL**: MySQL must be installed and accessible via the `mysql` and `mysqldump` commands.
- Appropriate permissions for the specified MySQL user (default is `root`).

## Installation

1. Clone or download this repository.
2. Ensure MySQL tools (`mysql` and `mysqldump`) are in your PATH.
3. Compile the project by running:
   ```bash
   cargo build --release
   ```
4. The executable will be available in target/release/.

# Usage

Run the program from the terminal:

```bash
./target/release/executable_name [--port PORT]
```

The default port is 3306. To specify a different port, use the --port argument

```bash
./target/release/executable_name --port 3307
```

Backups will be saved in /var/backups/mysql/ with the database name (e.g., database_name.sql).

# Configuration

- **MySQL User**: Currently set to `root`. Modify the `mysql_user` variable in the code if needed.
- **Backup Directory**: Set to `/var/backups/mysql/`. Change the `backup_dir` variable for a different location.
- **Default Port**: Set to `3306`. Adjust the `port` variable in the code for a different default.

## Notes
- Ensure the MySQL user has permissions to run `SHOW DATABASES` and `mysqldump`.
- If password authentication is required, modify the code to include the `-p` option and handle the password securely (e.g., via environment variables).
- This tool does not compress `.sql` files. You can extend the code to add compression (e.g., `gzip`).

## Dependencies

- **Rust Standard Library (std)**:
  - `std::process::Command` for executing system commands.
  - `std::fs` for file system operations.
  - `std::path::Path` for directory existence checks.
  - `std::env` for parsing command-line arguments.

## Contributing
Feel free to submit a pull request or report issues in the repository if you’d like to contribute.
