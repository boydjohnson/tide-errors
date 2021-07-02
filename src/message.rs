use tide::{prelude::Serialize, Body, Response, ResponseBuilder};

pub enum Message<T, E> {
    /// 200 Status
    Ok(T),
    /// 201 Status
    Created(T),
    /// 202 Status
    Accepted(T),
    /// 400 Status
    BadRequest(E),
    /// 401 Status
    Unauthorized(E),
    /// 403 Status
    Forbidden(E),
    /// 402 Status
    PaymentRequired(E),
    /// 404 Status
    NotFound(E),
    /// 409 Status
    Conflict(E),
    /// 429 Status
    TooManyRequests(E),
}

impl<T, E> Message<T, E> {
    pub fn ok(item: T) -> Self {
        Message::Ok(item)
    }

    pub fn created(item: T) -> Self {
        Message::Created(item)
    }

    pub fn accepted(item: T) -> Self {
        Message::Accepted(item)
    }

    pub fn bad_request(error: E) -> Self {
        Message::BadRequest(error)
    }

    pub fn unauthorized(error: E) -> Self {
        Message::Unauthorized(error)
    }

    pub fn forbidden(error: E) -> Self {
        Message::Forbidden(error)
    }

    pub fn payment_required(error: E) -> Self {
        Message::PaymentRequired(error)
    }

    pub fn not_found(error: E) -> Self {
        Message::NotFound(error)
    }

    pub fn conflict(error: E) -> Self {
        Message::Conflict(error)
    }

    pub fn too_many_requests(error: E) -> Self {
        Message::TooManyRequests(error)
    }
}

impl<T: Serialize, E: Serialize> From<Message<T, E>> for Result<tide::Response, tide::Error> {
    fn from(other: Message<T, E>) -> Self {
        use Message::*;
        match other {
            Ok(item) => add_body(ok(), item),
            Created(item) => add_body(created(), item),
            Accepted(item) => add_body(accepted(), item),
            BadRequest(error) => add_body(bad_request(), error),
            Unauthorized(error) => add_body(unauthorized(), error),
            Forbidden(error) => add_body(forbidden(), error),
            PaymentRequired(error) => add_body(payment_required(), error),
            NotFound(error) => add_body(not_found(), error),
            Conflict(error) => add_body(conflict(), error),
            TooManyRequests(error) => add_body(too_many_requests(), error),
        }
    }
}

fn add_body<S: Serialize>(
    builder: ResponseBuilder,
    item: S,
) -> Result<tide::Response, tide::Error> {
    Ok(builder.body(Body::from_json(&item)?).build())
}

fn ok() -> ResponseBuilder {
    Response::builder(200)
}

fn created() -> ResponseBuilder {
    Response::builder(201)
}

fn accepted() -> ResponseBuilder {
    Response::builder(202)
}

fn bad_request() -> ResponseBuilder {
    Response::builder(400)
}

fn unauthorized() -> ResponseBuilder {
    Response::builder(401)
}

fn payment_required() -> ResponseBuilder {
    Response::builder(402)
}

fn forbidden() -> ResponseBuilder {
    Response::builder(403)
}

fn not_found() -> ResponseBuilder {
    Response::builder(404)
}

fn conflict() -> ResponseBuilder {
    Response::builder(409)
}

fn too_many_requests() -> ResponseBuilder {
    Response::builder(429)
}
