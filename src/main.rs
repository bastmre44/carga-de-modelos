mod framebuffer;
mod obj_loader;

fn main() {
    let model = match obj_loader::load_obj("Nave_final.obj") {
        Ok(model) => model,
        Err(error) => {
            eprintln!("Error al cargar el modelo: {error}");
            std::process::exit(1);
        }
    };

    println!("Modelo cargado correctamente");
    println!("Vértices: {}", model.vertices.len());
    println!("Triángulos: {}", model.faces.len());
}
