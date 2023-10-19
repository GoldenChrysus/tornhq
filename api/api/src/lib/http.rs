use actix_web::HttpResponse;

pub fn make_json_response<T: serde::Serialize>(output: T) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(output)
}
