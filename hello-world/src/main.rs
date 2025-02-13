use zero2prod::run;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    run().await
}


// #[cfg(test)]
// mod test {
//     use crate::health_check;

//     #[tokio::test]
//     async fn health_check_succeeds() {
//         let response = health_check().await;


//         assert!(response.status().is_success());

//     }
// }