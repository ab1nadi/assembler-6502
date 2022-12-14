use std::error::Error;
use std::fmt;

#[macro_export]
macro_rules! unwrap_result_or_return_err {
    ( $e:expr, $s:expr) => {
        match $e {
            Ok(v) => v,
            Err(_) =>  return Err(GeneralError<T>::new($s)),
        }
    }
}

// General Error
// an error type
// i can return from 
// anything, has a from so 
// I know wherer the error came from 
#[derive(Debug)]
#[derive(Clone)]
pub struct GeneralError
{
    pub from: String,
    pub details: String
}

impl GeneralError {
    pub fn new(msg: &str, frm: &str) -> GeneralError {
        GeneralError
        {
            from: frm.to_string(),
            details: msg.to_string()
        }
    }
}

impl Error for GeneralError  {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"from: {}, \n{}",self.from, self.details)
    }
}


