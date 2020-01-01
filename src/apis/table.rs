pub async fn table_update(info: actix_web::web::Json<crate::models::TableUpdateRequest>) -> std::io::Result<actix_web::web::HttpResponse> {
    let body = 
        crate::models::TableUpdateResponse {
            version: "1.0.0".to_owned(),
            table: crate::models::ErrorTable {
                name: "table".to_owned(),
                cells: vec! {
                    crate::models::ErrorCell {
                        coord: crate::models::Coord { row: 1, col: 2 },
                        error: crate::models::Error { code: 10123, message: "nuclear hazard detected".to_owned() },
                    },
                },
            }
        };
    Ok(actix_web::web::HttpResponse::Ok()
       .set(actix_http::http::header::ContentType::json())
       .json(body))
    
}

#[cfg(test)]
mod tests {
    
    #[actix_rt::test]
    async fn unit_table_update() {
        let input =
            crate::models::TableUpdateRequest {
                version: "1.0.0".to_owned(),
                table: crate::models::DataTable {
                    name: "table".to_owned(),
                    cells: vec! {
                        crate::models::DataCell {
                            coord: crate::models::Coord { row: 1, col: 2 },
                            value: "Bart Simpson".to_owned(),
                        },
                    },
                },
            };
        let output =
            crate::models::TableUpdateResponse {
                version: "1.0.0".to_owned(),
                table: crate::models::ErrorTable {
                    name: "table".to_owned(),
                    cells: vec! {
                        crate::models::ErrorCell {
                            coord: crate::models::Coord { row: 1, col: 2 },
                            error: crate::models::Error { code: 10123, message: "nuclear hazard detected".to_owned() },
                        },
                    },
                }
            };

        let response = crate::apis::table::table_update(actix_web::web::Json(input)).await.unwrap();
        assert_eq!(response.status(), actix_http::http::StatusCode::OK);
    }

}
