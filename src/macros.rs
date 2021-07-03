#[macro_export]
macro_rules! convert {
    ($res:ident, ok => $f:expr, $($conversion:expr),+) => {
        {
            use crate::conversions::Converter;
            let mut converter = Converter::default();

            $(
                converter.add_conversion($conversion);
            )*

            converter.execute($res, $f)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{conversions::ConversionHandler, convert, message::Message};

    #[derive(Debug, PartialEq)]
    enum Error2 {
        UserError,
        ServerError,
    }

    #[derive(Debug, PartialEq)]
    struct Error {
        error: String,
    }
    struct UserErrorHandler;

    impl Default for UserErrorHandler {
        fn default() -> Self {
            UserErrorHandler
        }
    }

    impl<T> ConversionHandler<T, Error, Error2> for UserErrorHandler {
        fn possibly_convert(
            &mut self,
            res: &Result<T, Error2>,
        ) -> Option<Result<Message<T, Error>, Error2>> {
            match res {
                Err(Error2::UserError) => Some(Ok(Message::bad_request(Error {
                    error: "User error".into(),
                }))),
                Err(Error2::ServerError) => None,
                Ok(_) => None,
            }
        }
    }

    #[test]
    fn test_basic() {
        let res: Result<usize, Error2> = Err(Error2::UserError);

        assert_eq!(
            convert!(res, ok => Message::created, UserErrorHandler::default()),
            Ok(Message::<usize, _>::bad_request(Error {
                error: "User error".into()
            }))
        );
    }

    #[test]
    fn test_not_error() {
        let res = Ok(2);

        assert_eq!(
            convert!(res, ok => Message::created, UserErrorHandler::default()),
            Ok(Message::created(2))
        );
    }

    #[test]
    fn test_server_error() {
        let res: Result<usize, _> = Err(Error2::ServerError);

        assert_eq!(
            convert!(res, ok => Message::created, UserErrorHandler::default()),
            Err(Error2::ServerError)
        );
    }
}
