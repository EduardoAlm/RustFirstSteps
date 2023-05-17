use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
mod gcd;
use gcd::greatest_common_divisor;

/// tells serde to look at this when the prog is compile and generate code to parse a value of this type from data recieved from post reqs.
/// there is also de inverse Serialize which does the opposive is converts Rust values to an strutured format (json etc...).


#[derive(Deserialize)] 
struct GcdParameters{
    n: u64,
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });
    
    ///the || is a closure expression which is a value which can be called as an expression and inside {} is the body and if arguments where required would be between ||, 
    ///more on this each thread created which to handle requests will call our closure and get aa fresh copy of the app value which indicades handling and routing.
   
    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run().expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            ///rust raw string 
            r#" 
            <title>GCD Calculator</title>
            <form action ="/gcd" method="post">
            <input type="text" name="n" />
            <input type="text" name="m" />
            <button type="submit">Compute GCD</button>
            </form>
            "#,
        )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing GCD with zero its kinda nonsense dont u think?");
    }

    let response = 
        format!("The greatest common divisor of the numbers {} and {} is <b>{}<b>\n", 
            form.n, 
            form.m, 
            greatest_common_divisor(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}


