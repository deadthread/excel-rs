use std::io::{BufReader, Read, Seek};

use error::Result;
use minidom::Element;
use zip::{read::ZipFile, ZipArchive};

mod error;
mod ns;
mod element;

pub struct WorkBook<SR>(ZipArchive<SR>);

impl<SR: Seek + Read> WorkBook<SR> {
    pub fn from_seek_reader(input: SR) -> Result<Self> {
        ZipArchive::new(input).map(Self).map_err(Into::into)
    }

    fn get_xml_file(&mut self, path: &str) -> Result<Element> {
        let file = self.read_file(path).map(BufReader::new)?;
        minidom::Element::from_reader(file).map_err(Into::into)
    }

    fn read_file(&mut self, path: &str) -> Result<ZipFile> {
        self.0.by_name(path).map_err(Into::into)
    }
}
