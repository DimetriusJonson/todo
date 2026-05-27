use std::{borrow::Cow, collections::BTreeMap};

use axum::extract::{FromRequest, Request, rejection::JsonRejection};
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};

use crate::util::app_error::AppError;

/*
pub struct AppJson<T>(pub T);

impl<S, T> FromRequest<S> for AppJson<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let req = Request::from_parts(parts, body);

        let value = axum::Json::<T>::from_request(req, state).await?;
        return Ok(Self(value.0));
    }
}
*/

pub struct ValidJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidJson<T>
where
    T: Validate,
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let req = Request::from_parts(parts, body);

        let value = axum::Json::<T>::from_request(req, state).await?;
        let validate_result = value.0.validate();
        if let Err(validation_errors) = validate_result {
            return Err(transform_validation_errors(validation_errors))?;
        }

        Ok(Self(value.0))
    }
}

fn transform_validation_errors(validation_errors: ValidationErrors) -> ValidationErrors {
    let mut errors_map = validation_errors.0.clone();
    for (key, kind) in validation_errors.0 {
        match kind {
            ValidationErrorsKind::Struct(validation_errors) => {
                errors_map.insert(
                    key,
                    ValidationErrorsKind::Struct(Box::new(transform_validation_errors(
                        *validation_errors,
                    ))),
                );
            }
            ValidationErrorsKind::List(btree_map) => {
                let mut list_errors_map: BTreeMap<usize, Box<ValidationErrors>> = BTreeMap::new();
                for (i, validation_errors) in btree_map {
                    list_errors_map
                        .insert(i, Box::new(transform_validation_errors(*validation_errors)));
                }

                errors_map.insert(key, ValidationErrorsKind::List(list_errors_map));
            }
            ValidationErrorsKind::Field(validation_errors) => {
                let mut errors: Vec<ValidationError> = Vec::new();
                for field_err in validation_errors {
                    let mut new_field = field_err.clone();
                    new_field.message = transform_error_message(field_err);
                    errors.push(new_field);
                }
                errors_map.insert(key, ValidationErrorsKind::Field(errors));
            }
        }
    }
    ValidationErrors(errors_map)
}

fn transform_error_message(field_err: ValidationError) -> Option<Cow<'static, str>> {
    if field_err.message.is_some() {
        return field_err.message;
    }

    let params = field_err.params;
    let min = params.get("min");
    let max = params.get("max");

    match (field_err.code.as_ref(), min, max) {
        ("required", ..) => Some(Cow::Borrowed("Обязательно для заполнения")),
        ("length", Some(min), Some(max)) => {
            Some(Cow::Owned(format!("Длина от {} до {} символов", min, max)))
        }
        ("length", Some(min), None) => Some(Cow::Owned(format!("Длина минимум {} символа", min))),
        ("length", None, Some(max)) => Some(Cow::Owned(format!("Длина максимум {} символа", max))),
        _ => field_err.message,
    }
}
