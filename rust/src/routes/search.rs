use crate::mocks::api::PartSearchAPI;
use crate::search::highlighting::split_fragments;
use crate::{search::highlighting::Fragmented};
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize};
use serde_json::json;

// using get request for the search following RESTful convention.
// it is a search and I will not be modifying any state
pub async fn search_by_mpn(
    // Part api you can use for mocking response
    _part_api: web::Data<PartSearchAPI>,
    params: web::Query<SearchParams>,
) -> impl Responder {
    let search_query = params.query.clone();
    let mpns = _part_api.search("abx").unwrap();

    let mut search_results: Vec<Fragmented> = Vec::new();

    for mpn in mpns {
        search_results.push(split_fragments(&search_query, mpn.mpn.as_str()))
    }

    println!("results {:#?}", search_results);

    HttpResponse::Ok().json(json!({ "results": search_results }))
}

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    query: String,
}

#[cfg(test)]
mod tests {
    mod integration {
        use crate::mocks;
        use crate::routes::search::search_by_mpn;
        use actix_web::middleware::Logger;
        use actix_web::{test, web, App};

        #[actix_rt::test]
        async fn test_request() {
            let mut app = test::init_service(
                App::new()
                    .route("/search", web::get().to(search_by_mpn))
                    .wrap(Logger::default())
                    .data(mocks::api::PartSearchAPI {}),
            )
            .await;

            let mpn_req = test::TestRequest::get()
                .uri("/search?query=13")
                .to_request();
            let resp = test::call_service(&mut app, mpn_req).await;
            // println!("response {:?}", resp);
        }
    }
}
