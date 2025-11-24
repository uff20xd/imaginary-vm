type Byte = u8;

struct Loader {
    bytes: Vec<Byte>,
    error: Option<std::io::Error>,
}

struct Bucket {
    bucket: [Byte; 8],
    full: bool,
}
struct Memory<const SIZE: usize> {
    memory: [Bucket; SIZE],
}

struct Attribute {
    name: String,
    data: Vec<Byte>,
}

struct Struct {
    name: String,
    fields: Vec<Field>,
}

struct VmImaginary<const SIZE: usize> {
    memory: Memory<SIZE>,
    pc: usize,
}
