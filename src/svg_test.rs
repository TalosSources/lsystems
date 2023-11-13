use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

pub fn create_simple_svg() {
    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .move_by((-10, 10))
        .line_by((-20, 20));

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    let document = Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(path);

    svg::save("image.svg", &document).unwrap();
}