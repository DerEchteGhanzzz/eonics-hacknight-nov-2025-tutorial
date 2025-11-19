/*
    Go here after you've finished the exercises.

    The code below is for setting up your own webserver in rust.
    Maybe you want to host your own programming challange?
    Or, you might want to display your solutions for all to see?

    we use the package actix_web
    do: `cargo add actix_web` to add the webserver module to your Cargo.toml

    use actix_web::{App, HttpServer, get, post};

*/
    #[get("/")]
    fn service() -> impl Responder {
        HttpResponse::Ok().body(
            r#"
            <body>
                <div>
                    <h1>
                        Welcome to the Rusty Krab, may I take your order please?
                    </h1>
                </div>
            </body>
            "#)
    }

    // a struct that implements the Serializable and Deserialize Trait
    // this struct represents the api parameters you give to a request
    #[derive(Serializable, Deserialize)]
    struct HelloParams {
        name: String
    }

    // A get request with a parameter.
    // The parameter is parsed from a struct, in this case the HelloParams struct
    // /hello?name=YourName
    #[get("/get_hello")]
    fn hello(req: HttpRequest) -> impl Responder {
        
        let request = web::Query::<HelloParams>::from_query(req.query_string());
        match request {
            Err(_) => HttpResponse::BadRequest().body(format!("Wrong query parameters, use name=YourName")),
            Ok(params) => HttpResponse::Ok().body(format!("Hello {}", params.name)),
        }
        if request.is_err() {
            return ;
        }
    }

    // a post request
    // the body is parsed from the HelloParams struct as well
    // Body: { "name": "YourName" }
    #[post("/post_hello")]
    fn share_pizza(payload: web::Payload) {
        match handle_post::<HelloParams>(payload).await {
            Err(msg) => HttpResponse::NotAcceptable().body(msg),
            Ok(body) => HttpResponse::Ok().body(body),
        }
    }

    const MAX_SIZE: usize = 262_144; // max payload size is 256k

    // helper function to handle unwrapping a body safely
    // T is a generic type. Deserialize means it should be any T where Deserialize is implemented
    pub async fn handle_post<T: for<'a> Deserialize<'a>>(mut payload: web::Payload) -> Result<T, Error> {
        // payload is a stream of Bytes objects
        let mut body = web::BytesMut::new();
        while let Some(chunk) = payload.next().await {
            let chunk = chunk?;
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                return Err(error::ErrorBadRequest("overflow"))
            }
            body.extend_from_slice(&chunk);
        }

        // body is loaded, now we can deserialize serde-json
        Ok(serde_json::from_slice::<T>(&body)?)
    }

    async fn main() -> Result<()> {
            HttpServer::new(|| App::new()
        .service(index)
        .service(hello)
        .service(post)
        )
            .bind("localhost:80")?
            .run()
            .await
    }