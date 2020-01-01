mod apis {
    mod table {

        #[actix_rt::test]
        async fn integration_table_update() {
            let input =
                poc_rust_actix::models::TableUpdateRequest {
                    version: "1.0.0".to_owned(),
                    table: poc_rust_actix::models::DataTable {
                        name: "table".to_owned(),
                        cells: vec! {
                            poc_rust_actix::models::DataCell {
                                coord: poc_rust_actix::models::Coord { row: 1, col: 2 },
                                value: "Bart Simpson".to_owned(),
                            },
                        },
                    },
                };
            let expected =
                poc_rust_actix::models::TableUpdateResponse {
                    version: "1.0.0".to_owned(),
                    table: poc_rust_actix::models::ErrorTable {
                        name: "table".to_owned(),
                        cells: vec! {
                            poc_rust_actix::models::ErrorCell {
                                coord: poc_rust_actix::models::Coord { row: 1, col: 2 },
                                error: poc_rust_actix::models::Error { code: 10123, message: "nuclear hazard detected".to_owned() },
                            },
                        },
                    },
                };

            use poc_rust_actix;

            let mut app =
                actix_web::test::init_service(
                    actix_web::App::new()
                        .service(
                            actix_web::web::resource("/table_update")
                                .route(actix_web::web::post()
                                       .to(poc_rust_actix::apis::table::table_update)))).await;

            let endpoint = actix_web::test::TestRequest::default();
            let request = endpoint
                .method(actix_http::http::Method::POST)
                .uri("/table_update")
                .set(actix_http::http::header::ContentType::json())
                .set_json(&input)
                .to_request();

            use actix_service::Service;

            let response = app.call(request).await.unwrap();
            assert!(response.status().is_success());

            /*
            let client = awc::Client::default();
            let request =
                client
                .post("/table_update")
                .set(actix_http::http::header::ContentType::json())
                .send_json(&payload);
             */
            
            /*
              match result {
                  Ok(actual) =>
                      if(actual == expected) {
                          Ok("Request succeeded but returned unexpected results")
                      } else {
                          Err("Request succeeded but returned unexpected results") },
                  Err(e) => Err(e)
              })
          .map(|_| println!("Request succeeded."))
          .map_err(|_| { println!("Request failed."); assert!(false) });
          */
        }
    }
}
