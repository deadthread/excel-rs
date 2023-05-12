use std::{path::Path, str::FromStr};

use minidom::Element;

use crate::{
    error::{BadAttr, Error, NoAttr, Result},
    ns::DEFAULT_NS,
};

pub trait ElementUtils {
    fn get_child_d_ns(&self, name: &'static str) -> Result<&Element>;
    fn get_child_recursive(&self, path: &'static str) -> Result<&Element>;
    fn parse_attr<T: FromStr>(&self, name: &'static str) -> Result<T>
    where
        <T as FromStr>::Err: std::error::Error + 'static;
}

impl ElementUtils for Element {
    fn get_child_d_ns(&self, name: &'static str) -> Result<&Element> {
        self.get_child(name, DEFAULT_NS)
            .ok_or(Error::NoElement(name))
    }

    fn get_child_recursive(&self, path: &'static str) -> Result<&Element> {
        let mut element = self;
        let path = Path::new(path);

        for p in path {
            let p = p.to_str().unwrap();
            element = element.get_child_d_ns(p)?;
        }

        Ok(element)
    }

    fn parse_attr<T: FromStr>(&self, name: &'static str) -> Result<T>
    where
        <T as FromStr>::Err: std::error::Error + 'static,
    {
        let value = self
            .attr(name)
            .ok_or_else(|| NoAttr::new(name, self.name()))?;

        value
            .parse::<T>()
            .map_err(|e| BadAttr::new::<_, T>(name, value, e))
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use minidom::Element;

    use super::*;

    const ELEMENT_SAMPLE: &str = r#"<a xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><b><c></c></b></a>"#;

    fn init_element() -> Element {
        Element::from_str(ELEMENT_SAMPLE).unwrap()
    }

    #[test]
    fn get_child_d_ns_test() {
        let element = init_element();
        let child = element.get_child_d_ns("b").unwrap();

        assert_eq!(child.name(), "b")
    }

    #[test]
    fn get_child_recursive_test() {
        let element = init_element();
        let child = element.get_child_recursive("b/c").unwrap();

        assert_eq!(child.name(), "c")
    }
}
