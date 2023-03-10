fn main() {
    let cpl = CPL::from_file("./CPL_2acb512a-d3c4-4dc9-9a3c-946dcbaa39bf.xml");
    println!("{:?}", cpl);
}

// fill this struct intermediate values
#[derive(Debug)]
struct CPL {}

impl CPL {
    // parse XML here
    fn from_file(path: &str) -> CPL {
        CPL {}
    }

    // return annotation text field
    fn annotation_text(&self) -> String {
        todo!()
    }

    // return main picture duration in frames
    fn main_picture_duratuion(&self) -> u64 {
        todo!()
    }

    // return main picture duration of a frame in seconds (1.0 / frame rate)
    fn main_picture_frame_duratuion(&self) -> f64 {
        todo!()
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
