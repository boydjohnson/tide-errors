use crate::{conversions::ConversionHandler, message::Message};
use std::borrow::Cow;

const UNIQUE_CONSTRAINT_CODE: &str = "23505";
const FOREIGN_KEY_CONSTRAINT_CODE: &str = "23503";

pub trait DatabaseCode<T, E> {
    const CODE: &'static str;

    fn to_message(error: E) -> Message<T, E>;
}

pub struct UniqueConstraint;

impl<T, E> DatabaseCode<T, E> for UniqueConstraint {
    const CODE: &'static str = UNIQUE_CONSTRAINT_CODE;

    fn to_message(error: E) -> Message<T, E> {
        Message::conflict(error)
    }
}

pub struct ForeignKeyConstraint;

impl<T, E> DatabaseCode<T, E> for ForeignKeyConstraint {
    const CODE: &'static str = FOREIGN_KEY_CONSTRAINT_CODE;

    fn to_message(error: E) -> Message<T, E> {
        Message::bad_request(error)
    }
}

pub struct DatabaseConstraintConversion<O, D> {
    field: String,
    index: Option<String>,
    options: Option<O>,
    _phantom: std::marker::PhantomData<D>,
}

impl<T, E, O, D> ConversionHandler<T, E, sqlx::Error> for DatabaseConstraintConversion<O, D>
where
    (String, Option<O>): Into<E>,
    D: DatabaseCode<T, E>,
{
    fn possibly_convert(
        &mut self,
        res: &Result<T, sqlx::Error>,
    ) -> Option<Result<Message<T, E>, sqlx::Error>> {
        if let Err(sqlx::Error::Database(e)) = res {
            if e.code() == Some(Cow::from(D::CODE))
                && self.index.is_some()
                && self.index.as_deref() == e.constraint()
                || e.code() == Some(Cow::from(D::CODE)) && self.index.is_none()
            {
                return Some(Ok(D::to_message(
                    (self.field.clone(), self.options.take()).into(),
                )));
            }
        }

        None
    }
}
