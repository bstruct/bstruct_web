pub type BaseResult<T> = Result<T, Box<dyn std::error::Error>>;

pub trait ToBaseResult<T> {
    fn to_base_result(self) -> Result<T, Box<dyn std::error::Error>>;
}

impl<T, E> ToBaseResult<T> for std::result::Result<T, E>
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

// pub trait Clone<T> {
//     fn clone(&self) -> BaseResult<T>;
// }

// impl<T> Clone<T> for Result<T, Box<dyn std::error::Error>>
// where
//     T: std::clone::Clone,
// {
//     fn clone(&self) -> Self {
//         match self {
//             Self::Ok(arg0) => Self::Ok(arg0.clone()),
//             Self::Err(error) => Err(format!("{:?}", error).into()),
//         }
//     }
// }
