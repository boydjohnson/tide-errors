use crate::{conversions::ConversionHandler, message::Message};
use std::borrow::Cow;

const UNIQUE_CONSTRAINT_CODE: &str = "23505";

pub struct UniqueConstraintConversion<O> {
    field: String,
    index: Option<String>,
    options: Option<O>,
}

impl<T, E, O> ConversionHandler<T, E, sqlx::Error> for UniqueConstraintConversion<O>
where
    (String, Option<O>): Into<E>,
{
    fn possibly_convert(
        &mut self,
        res: &Result<T, sqlx::Error>,
    ) -> Option<Result<Message<T, E>, sqlx::Error>> {
        if let Err(sqlx::Error::Database(e)) = res {
            if e.code() == Some(Cow::from(UNIQUE_CONSTRAINT_CODE))
                && self.index.is_some()
                && self.index.as_deref() == e.constraint()
                || e.code() == Some(Cow::from(UNIQUE_CONSTRAINT_CODE)) && self.index.is_none()
            {
                return Some(Ok(Message::conflict(
                    (self.field.clone(), self.options.take()).into(),
                )));
            }
        }

        None
    }
}
