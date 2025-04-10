# db-backup-manager

A simple tool to back up all MySQL databases.

## Requirements

- MySQL `root` user.
- **No** authentication required.

## What does it do?

- Exports **all databases**.
- Compresses them into `.tar.gz` format.
- The generated file is named with the **date and time of the backup**.
- Backups are stored in the folder: `/var/backups/mysql`.


## Optional parameters

- `--port`: allows you to specify the MySQL server port. If not provided, the default port **3306** will be used.
- `--limit`: defines how many backups to keep. If not set, **no old backups will be deleted**. For example, `--limit 30` will keep the **30 most recent backups** and delete the older ones.


## Usage

### 1. Create a folder for the executable

You can do this in any directory, for example:

```bash
mkdir -p /root/bin/
```

### 2. Download the executable

Go to the [releases page](https://github.com/Hanhan1989/db-backup-manager/releases) and download the executable manually or with `curl`:

```bash
curl -LO https://github.com/Hanhan1989/db-backup-manager/releases/download/v2.0.0/db-backup-manager-Linux-v2.0.0
```

### 3. Set appropriate permissions

```bash
chown root:root db-backup-manager-Linux-v2.0.0
chmod 700 db-backup-manager-Linux-v2.0.0
```

### 4. Run it with `cron`

Example: run a backup **every day at midnight**, limiting to **30 backups** (older ones will be deleted automatically). If the port is `3306`, you can omit it.

```cron
# Execute the command crontab -e

0 0 * * * /root/bin/db-backup-manager --port 5555 --limit 30
```

---

## Development

If you're working on the code, after making changes, remember to compile:

```bash
cargo build
```

Or to generate an optimized executable:

```bash
cargo build --release
```

The generated executables will be in the `target` directory.

---

That's it! 🎉
