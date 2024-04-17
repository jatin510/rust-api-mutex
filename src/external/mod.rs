pub trait ExternalApi {
    fn get_new_access_token() -> Result<String, String>;
    fn get_info() -> Result<String, String>;
}

pub mod external;
