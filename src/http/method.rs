#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl AsRef<str> for Method {
    fn as_ref(&self) -> &str {
        match self {
            Method::Post => "POST",
            Method::Delete => "DELETE",
            Method::Get => "GET",
            Method::Put => "PUT",
        }
    }
}
