use std::fs;
use std::io::Read;
use std::path::Path;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vm = VmImaginary::<256>::new();
    let bucket = Bucket::default();
    let mut programm_loader = ProgrammLoader::new("tests/Add.class")?;
    dbg!(&programm_loader);
    dbg!(programm_loader.get_next_bytes(2));
    dbg!(programm_loader.get_next_bytes(2));
    Ok(())
}

type VmError = String;

type Byte = u8;

#[derive(Debug, Clone)]
struct ProgrammLoader {
    bytes: Vec<Byte>,
    counter: usize,
}

impl ProgrammLoader{
    pub fn new<P: AsRef<Path>>(file_name: P) -> Result<Self, std::io::Error> { 
        let mut bytes = Vec::new();
        let mut file = fs::File::open(file_name)?;
        _ = file.read_to_end(&mut bytes);
        Ok(Self{
            bytes,
            counter: 0,
        })
    }
    pub fn get_bytes(&self, from: usize, to: usize) -> &[Byte] {
        &self.bytes[from..to]
    }

    pub fn get_next_bytes(&mut self, n:usize) -> &[Byte] {
        let teva = &self.bytes[self.counter..self.counter + n];
        self.counter += n;
        teva
    }
}

struct Programm {}

#[derive(Debug, Clone, Copy, Default)]
struct Bucket {
    bucket: [Byte; 8],
    full: bool,
}

enum Const {
    String(String),
    NameIndex(u16),
    TypeAndNameIndex(u16),
    StringIndex(u16),
    DescIndex(u16),
    ClassIndex(u16),
}
impl Const {
    pub fn parse(loader: &ProgrammLoader) -> Result<Self, VmError> {
        todo!()
    }
    pub fn get_str(&self) -> Result<String, VmError> {
        match self {
            Const::String(str) => {return Ok(str.clone())},
            _ => {return Err("Not a String in Const::get_str".to_owned())},
        }
    }
}

type ConstPool = Vec<Const>;

struct Memory<const SIZE: usize> {
    memory: [Bucket; SIZE],
}

impl<const SIZE: usize> Memory<SIZE> {
    pub fn init() -> Self {
        Self { memory: [Bucket::default(); SIZE] }
    }
}

struct Field {
    flags: u16,
    name: String,
    descriptors: String,
    attributes: Vec<Attribute>,
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

