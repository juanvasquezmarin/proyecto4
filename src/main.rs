use actix_web::{web, App, HttpServer, Responder};

//Manejador de ruta

async fn hello()-> impl Responder{
    "hello, world!"
}

#[actix_web:: main]
async fn main()-> std::io::Result<()>{
    // Iniciar el servidor web
    HttpServer::new(||{
        App::new()
        //Ruta y su correspondiente manejador
        .route("/",web::get().to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// 