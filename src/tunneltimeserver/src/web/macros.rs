#[macro_export]
macro_rules! isetry {
    ($result:expr) => {
        match $result {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(err) => return $crate::web::types::five_hundred(err),
        }
    };
}

#[macro_export]
macro_rules! resulttry {
    ($result:expr) => {
        match $result {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(err) => match err {
                $crate::error::Error::NoSqlRows => {
                    return $crate::web::responses::resource_not_found()
                }
                _ => return $crate::web::types::five_hundred(err),
            },
        }
    };
}

#[macro_export]
macro_rules! rtry {
    ($result:expr) => {
        match $result {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(resp) => return resp,
        }
    };
}
