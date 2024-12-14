pub type BaseResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait ToBaseResult<T, E> {
    fn to_base_result(self) -> Result<T, Box<dyn std::error::Error>>;
}

impl<T, E> ToBaseResult<T, E> for std::result::Result<T, E>
where
    T: std::fmt::Debug,
    E: std::fmt::Debug,
{
    fn to_base_result(self) -> Result<T, Box<dyn std::error::Error>> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(format!("{:?}", error).into()),
        }
    }
}
