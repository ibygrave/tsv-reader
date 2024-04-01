use tsv_reader::err::Error;
use tsv_reader::reader::{Read, Walker};

#[derive(PartialEq, Debug, Read)]
struct Colour([u8; 3]);

#[derive(PartialEq, Debug, Read)]
struct Header<'a> {
    version: u32,
    title: &'a str,
    background: Colour,
}

#[derive(PartialEq, Debug, Read)]
enum Shape {
    Line(u32, u32, u32, u32),
    Circle(u32, u32, u32),
    Rectange(u32, u32, u32, u32),
}

#[derive(PartialEq, Debug, Read)]
struct Object {
    colour: Colour,
    shape: Shape,
}

fn parse_data(
    input: &[u8],
) -> Result<(Header<'_>, impl Iterator<Item = Object> + '_), Error> {
    let mut lines = core::str::from_utf8(input)?.split('\n');
    let header_line = lines.next().ok_or(Error)?;
    Ok((
        Walker::parse_one_line(header_line)?,
        lines.map_while(|line| Walker::parse_one_line(line).ok()),
    ))
}

const DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/data.tsv"));

fn main() {
    let (header, files) = parse_data(DATA).unwrap();
    let objects: Vec<Object> = files.collect();

    assert_eq!(
        header,
        Header {
            version: 1,
            title: "Example Title",
            background: Colour([255, 255, 255])
        }
    );
    println!("{:?}", header);
    println!("{:?}", objects);
}
