use crate::errors::FouineApiError;
use actix_web::{web, Responder};
use serde::Serialize;
use std::fmt::Debug;

#[derive(Serialize)]
pub struct Meta {
    length: usize,
}

#[derive(Serialize)]
pub struct Collection<To: Serialize> {
    pub data: Vec<To>,
    pub meta: Meta,
}

pub trait GenericTransformer<From, To: Debug> {
    fn transform(elem: From) -> To;
}

pub fn transform_collection<From, To: Debug + Serialize, T: GenericTransformer<From, To>>(
    array: Vec<From>,
) -> Result<impl Responder, FouineApiError> {
    let mut transformed_array: Vec<To> = vec![];

    for elem in array {
        transformed_array.push(T::transform(elem));
    }

    Ok(web::Json(Collection {
        meta: Meta {
            length: transformed_array.len(),
        },
        data: transformed_array,
    }))
}

pub fn transform_one<From, To: Debug + Serialize, T: GenericTransformer<From, To>>(
    elem: From,
) -> Result<impl Responder, FouineApiError> {
    Ok(web::Json(T::transform(elem)))
}
