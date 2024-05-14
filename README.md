# tsv-reader

Minimal parser for tab-seperated-value (TSV) data.
Usable in `no_std` and without `alloc`.
The supported data format differs from standard TSV
by omitting the header line, and allowing each line
to encode a different type.

### Example
```rust
// A TSV document where the first line is a `Header` and the rest are `Object`s.
const DOC: &[u8] = b"1\tExample Title\tFFFFFF
000000\tfalse\tLine\t0\t0\t500\t500
550055\ttrue\tCircle\t200\t300\t20
FF0055\tfalse\tRectangle\t100\t100\t200\t200";

#[derive(PartialEq, Debug, Read)]
struct Colour([u8; 3]);

#[derive(PartialEq, Debug, Read)]
struct Header<'doc> {
    version: u32,
    title: &'doc str,
    background: Colour,
}

#[derive(PartialEq, Debug, Read)]
enum Shape {
    Line(u32, u32, u32, u32),
    Circle(u32, u32, u32),
    Rectangle(u32, u32, u32, u32),
}

#[derive(PartialEq, Debug, Read)]
struct Object {
    colour: Colour,
    fill: bool,
    shape: Shape,
}

fn main() {
    let mut doc = Document::new(DOC).unwrap();
    let header: Header = doc.parse_one().unwrap();
    let objects: Result<Vec<Object>, Error> = doc.parse_iter().collect();
    let objects = objects.unwrap();

    assert_eq!(
        header,
        Header {
            version: 1,
            title: "Example Title",
            background: Colour([255, 255, 255])
        }
    );
    assert_eq!(objects.len(), 3);
    assert_eq!(
        objects[1],
        Object {
            colour: Colour([0x55, 0x00, 0x55]),
            fill: true,
            shape: Shape::Circle(200, 300, 20)
        }
    );
}
```
