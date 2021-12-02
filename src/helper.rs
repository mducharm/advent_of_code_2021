pub fn get_file_data_by_name(input_data: &[(&str, &str)], file_name: &str) -> String {
    let (_name, contents) = input_data.iter().find(|(name, _contents)| *name == file_name).unwrap().to_owned();

    contents.to_owned()
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("expected {0}")]
    Expected(&'static str),
}

#[macro_export]
#[allow(clippy::vec_init_then_push)]
macro_rules! include_input_data {
    ( $( $x:expr ),* ) => {
        {
            let mut input_files = Vec::new();

            $(
                input_files.push(($x, include_str!(concat!("./input_data/", $x, ".txt"))));
            )*

            input_files
        }
    };
}
