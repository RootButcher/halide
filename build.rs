
use halide::build::{Generator,GenBuilder};

fn main() {

    let Hal = GenBuilder::new(
        "/home/jacob/Desktop/Halide",
        "src/gens"
    );
       // .out_dir("src/rs");


    let gen = Hal.new_gen("iir_blur".to_string());

    assert!(gen.compile().status.success());

    assert!(gen.run_gen().status.success());

    assert!(gen.bind().is_ok());

    assert!(gen.rename_move().is_ok());

}