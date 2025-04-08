use std::process::Command;
use std::fs;
use std::path::PathBuf;
use std::env;
use chrono::Local;
use tempfile::TempDir;

fn main() {
    let backup_dir = "/var/backups/mysql";
    let mysql_user = "root";

    // Obtener el puerto desde los argumentos
    let args: Vec<String> = env::args().collect();
    let mut port = "3306".to_string(); // Valor por defecto

    if let Some(index) = args.iter().position(|x| x == "--port") {
        if let Some(port_value) = args.get(index + 1) {
            port = port_value.clone();
        }
    }

    // Crear directorio destino si no existe
    if !PathBuf::from(backup_dir).exists() {
        fs::create_dir_all(backup_dir).expect("No se pudo crear el directorio de backup");
    }

    // Crear carpeta temporal
    let temp_dir = TempDir::new().expect("No se pudo crear el directorio temporal");
    let temp_path = temp_dir.path();

    // Obtener lista de bases de datos
    let output = Command::new("mysql")
        .arg("-u").arg(mysql_user)
        .arg("-P").arg(&port)
        .arg("-e").arg("SHOW DATABASES;")
        .output()
        .expect("Fallo al obtener la lista de bases de datos");

    if !output.status.success() {
        eprintln!("Error ejecutando SHOW DATABASES");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return;
    }

    let db_list = String::from_utf8_lossy(&output.stdout);

    for db in db_list.lines().skip(1) {
        if ["information_schema", "performance_schema", "mysql", "sys", "test"].contains(&db) {
            continue;
        }

        let backup_file = temp_path.join(format!("{}.sql", db));
        println!("Exportando {}...", db);

        let dump_status = Command::new("mysqldump")
            .arg("-u").arg(mysql_user)
            .arg("-P").arg(&port)
            .arg(db)
            .arg("--result-file").arg(&backup_file)
            .status()
            .expect("Fallo al ejecutar mysqldump");

        if dump_status.success() {
            println!("Backup de {} completado en {:?}", db, backup_file);
        } else {
            eprintln!("Error al exportar la base de datos {}", db);
        }
    }

    // Crear nombre del fichero comprimido
    let now = Local::now();
    let timestamp = now.format("%Y%m%d_%H%M%S").to_string();
    let tar_path = format!("{}/backup_mysql_{}.tar.gz", backup_dir, timestamp);

    println!("Comprimiendo backups en {}", tar_path);

    let tar_status = Command::new("tar")
        .arg("-czf")
        .arg(&tar_path)
        .arg("-C")
        .arg(temp_path)
        .arg(".") // Comprimir todo lo que hay dentro del temp_dir
        .status()
        .expect("Fallo al comprimir los backups");

    if tar_status.success() {
        println!("Backup comprimido creado en {}", tar_path);
        // `temp_dir` se elimina automáticamente al salir del scope
    } else {
        eprintln!("Fallo al crear el archivo comprimido");
    }
}
