use actix_web::{get, Error, HttpResponse};

#[get("/tea")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::ImATeapot().body(
        r#"
           .------.____
        .-'       \ ___)
     .-'         \\\
  .-'        ___  \\)
.-'          /  (\  |)
        __  \  ( | |
       /  \  \__'| |
      /    \____).-'
    .'       /   |
   /     .  /    |
 .'     / \/     |
/      /   \     |
      /    /    _|_
      \   /    /\ /\
       \ /    /__v__\
        '    |       |
             |     .#|
             |#.  .##|
             |#######|
             |#######|"#,
    ))
}
