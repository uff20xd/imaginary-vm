fn main() {
    let vm = VmImaginary::<256>::new();
    let bucket = Bucket::default();
    dbg!(bucket);
}

type Byte = u8;

struct ProgrammLoader {
    bytes: Vec<Byte>,
    counter: usize,
}

impl ProgrammLoader{
    pub fn new() -> Result<Self, std::io::Error> { 
        let mut bytes = Vec::new();
        Ok(Self{
            bytes,
            counter: 0,
        })
    }
    pub fn get_bytes(&self, n: usize) -> &[Byte] {
        &self.bytes[self.counter..n]
    }
}

struct Programm {}

#[derive(Debug, Clone, Copy, Default)]
struct Bucket {
    bucket: [Byte; 8],
    full: bool,
}


struct Memory<const SIZE: usize> {
    memory: [Bucket; SIZE],
}

impl<const SIZE: usize> Memory<SIZE> {
    pub fn init() -> Self {
        Self { memory: [Bucket::default(); SIZE] }
    }
}

struct Field {}

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
    p: Programm,
}

impl<const SIZE: usize> VmImaginary<SIZE> {
    pub fn new() -> Self {
        Self {
            memory: Memory::<SIZE>::init(),
            pc: 0,
            p: Programm {},
        }
    }
    pub fn exec_program(programm: Programm) -> Result<(), Box<dyn std::error::Error>> { todo!() }
}

