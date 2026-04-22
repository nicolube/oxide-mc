
mod models;
mod functions;
mod launcher;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn download() {
        let ruta_juego = std::path::Path::new("test_minecraft");
        let java_path = ruta_juego.join("runtime/jdk-17.0.10+7/bin/java.exe");

        // 1. Obtener los datos
        let manifest = functions::get_manifest().await.unwrap();

        // 2. Descargar Librerías
        functions::download_libraries(&manifest, ruta_juego).await.unwrap();

        // 3. Descargar el Cliente (EL PASO QUE FALTA)
        functions::download_client(&manifest, ruta_juego).await.unwrap();

        functions::download_assets(&manifest, ruta_juego).await.unwrap();

        println!("¡All done successfully!");
    }

    #[tokio::test]
    async fn launch() {
        let game_path = std::path::Path::new("test_minecraft");
        let java_path = game_path.join("runtime/jdk-17.0.10+7/bin/java.exe");

        // 1. Obtener los datos
        let manifest = functions::get_manifest().await.unwrap();

        let cp = functions::gen_classpath(&manifest, game_path);
        println!("Classpath generated ({} characters)", cp.len());

        match launcher::lanzar_juego(&manifest, game_path, &java_path, "S3ffl_Dev", cp) {
            Ok(mut child) => {
                println!("Windows opened! Waiting for game to close...");
                // Esto hace que el test espere a que cierres el juego para terminar
                let status = child.wait().expect("Failed waiting game");
                println!("The game exited with status: {}", status);
            },
            Err(e) => {
                panic!("Error launching game: {}", e);
            }
        }
    }


}
