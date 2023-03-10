extern crate yaserde_derive;

use xml_schema_derive::XmlSchema;

fn main() {
    let cpl = CPL::from_file("./CPL_2acb512a-d3c4-4dc9-9a3c-946dcbaa39bf.xml");
    println!("{:?}", cpl);
}

// fill this struct intermediate values
#[derive(Debug, XmlSchema)]
#[xml_schema(
    source = "schema/SMPTE-429-7-2006-CPL.xsd",
    target_prefix = "my_prefix",
    store_generated_code = "cpl_generated.rs"
)]
struct CPL {
    cpl: types::CompositionPlaylistType,
}

impl CPL {
    // parse XML here
    fn from_file(path: &str) -> CPL {
        let xml_str = std::fs::read_to_string(path).unwrap();
        let cpl: types::CompositionPlaylistType = yaserde::de::from_str(&xml_str).unwrap();
        CPL { cpl }
    }

    // return annotation text field
    fn annotation_text(&self) -> String {
        match self.cpl.annotation_text {
            Some(ref annotation_text) => annotation_text.content.clone(),
            None => String::new(),
        }
    }

    // return main picture duration in frames
    fn main_picture_duratuion(&self) -> i64 {
        match self.cpl.reel_list.reels.iter().next() {
            Some(reel) => match reel.asset_list.main_picture {
                Some(ref main_picture) => main_picture.content.content.duration.unwrap(),
                None => 0,
            },
            None => 0,
        }
    }

    // return main picture duration of a frame in seconds (1.0 / frame rate)
    fn main_picture_frame_duratuion(&self) -> f64 {
        match self.cpl.reel_list.reels.iter().next() {
            Some(reel) => match reel.asset_list.main_picture {
                Some(ref main_picture) => {
                    let frame_rate_str = main_picture.content.content.edit_rate.content.as_str();
                    let mut nominator = 0i32;
                    let mut denominator = 0i32;
                    scanf::sscanf!(frame_rate_str, "{} {}", nominator, denominator).unwrap();
                    nominator as f64 / denominator as f64
                }
                None => 0.0,
            },
            None => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CPL;

    #[test]
    fn annotation_text() {
        let cpl = CPL::from_file("./CPL_2acb512a-d3c4-4dc9-9a3c-946dcbaa39bf.xml");
        assert_eq!(cpl.annotation_text(), "HDR Demo");
    }

    #[test]
    fn main_picture_duratuion() {
        let cpl = CPL::from_file("./CPL_2acb512a-d3c4-4dc9-9a3c-946dcbaa39bf.xml");
        assert_eq!(cpl.main_picture_duratuion(), 171);
    }

    #[test]
    fn main_picture_frame_duratuion() {
        let cpl = CPL::from_file("./CPL_2acb512a-d3c4-4dc9-9a3c-946dcbaa39bf.xml");
        assert_eq!(1.0 / cpl.main_picture_frame_duratuion(), 1.0 / 24.0);
    }
}
