error_chain!{
        foreign_links {
            Io(::std::io::Error);
            SerdeXML(::serde_xml_rs::Error);
            Parse(::std::num::ParseIntError);
            Fgen(::fgen::errors::Error);
        }
    }
