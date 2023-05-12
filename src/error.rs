use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
    #[error(transparent)]
    Xml(#[from] minidom::Error),
    #[error("No {0} element found")]
    NoElement(&'static str),
    #[error(transparent)]
    NoAttr(#[from] NoAttr),
    #[error(transparent)]
    BadAttr(#[from] BadAttr)
}

#[derive(Debug, Error)]
#[error("No {attr_name} attribute found in element {element_name}")]
pub struct NoAttr {
    attr_name: &'static str,
    element_name: &'static str
}

impl NoAttr {
    pub fn new(attr_name: &'static str, element_name: &str) -> Self {
        let element_name_static = Box::leak(element_name.to_string().into_boxed_str());
        Self { attr_name, element_name: element_name_static }
    }
}

#[derive(Debug, Error)]
#[error("Error parsing value {value:?} (from attr {name}) to {parse_to}")]
pub struct BadAttr {
    name: &'static str,
    value: &'static str,
    parse_to: &'static str,
    source: Box<dyn std::error::Error>
}

impl BadAttr {
    pub fn new<E: std::error::Error + 'static, T>(name: &'static str, value: &str, source: E) -> Self {
        let value_static = Box::leak(value.to_string().into_boxed_str());
        let parse_to = std::any::type_name::<T>();
        let source = Box::new(source);

        Self { name, value: value_static, parse_to, source }
    }
}