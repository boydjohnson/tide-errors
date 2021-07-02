use crate::message::Message;

#[cfg(feature = "sqlx")]
pub mod database;

pub trait ConversionHandler<T, E1, E2> {
    fn possibly_convert(&mut self, res: &Result<T, E2>) -> Option<Result<Message<T, E1>, E2>>;
}

pub struct Converter<T, E1, E2> {
    conversions: Vec<Box<dyn ConversionHandler<T, E1, E2>>>,
}

impl<T, E1, E2> Default for Converter<T, E1, E2> {
    fn default() -> Self {
        Converter {
            conversions: vec![],
        }
    }
}

impl<T, E1, E2> Converter<T, E1, E2> {
    pub fn add_conversion<C: ConversionHandler<T, E1, E2> + 'static>(&mut self, conversion: C) {
        self.conversions.push(Box::new(conversion));
    }

    pub fn execute<F: Fn(T) -> Message<T, E1>>(
        &mut self,
        res: Result<T, E2>,
        f: F,
    ) -> Result<Message<T, E1>, E2> {
        for converter in &mut self.conversions {
            if let Some(msg) = converter.possibly_convert(&res) {
                return msg;
            }
        }

        res.map(f)
    }
}
