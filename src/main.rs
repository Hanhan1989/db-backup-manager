use std::process::Command;
use std::fs;
use std::path::Path;
use std::env;

fn main() {
    let backup_dir = "/var/backups/mysql/";
    let mysql_user = "root";
    
    // Obtener el puerto desde los argumentos
    let args: Vec<String> = env::args().collect();
    let mut port = "3306".to_string(); // Valor por defecto

    if let Some(index) = args.iter().position(|x| x == "--port") {
        if let Some(port_value) = args.get(index + 1) {
            port = port_value.clone();
        }
    }

    // Crear directorio si no existe
    if !Path::new(backup_dir).exists() {
        fs::create_dir_all(backup_dir).expect("No se pudo crear el directorio de backup");
    }

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
    
    for db in db_list.lines().skip(1) {  // Saltamos la primera línea (header)
        if db == "information_schema" || db == "performance_schema" || db == "mysql" || db == "sys" {
            continue; // No hacer backup de bases de datos del sistema
        }

        let backup_file = format!("{}/{}.sql", backup_dir, db);
        println!("Exportando {}...", db);

        let dump_status = Command::new("mysqldump")
            .arg("-u").arg(mysql_user)
            .arg("-P").arg(&port)
            .arg(db)
            .arg("--result-file").arg(&backup_file)
            .status()
            .expect("Fallo al ejecutar mysqldump");

        if dump_status.success() {
            println!("Backup de {} completado en {}", db, backup_file);
        } else {
            eprintln!("Error al exportar la base de datos {}", db);
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
}
