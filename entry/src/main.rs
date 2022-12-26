fn main() {
    println!("hello");
    kkbt::show_file_hex("data/1",999).unwrap();
   // SKV::init("1", "2").unwrap();
    let skv = SKV::config("data/index", "data/data");
    dbg!(skv.get("k1").unwrap());
}
struct SKV<'a> {
    index_file: &'a str,
    data_file: &'a str,
}
impl<'a> SKV<'a> {
    // create two files
    #[allow(dead_code)]
    fn init(index_file:&'a str,data_file:&'a str) -> std::io::Result<SKV<'a>> {
        std::fs::File::create(index_file)?;
        std::fs::File::create(data_file)?;
        Ok(Self { index_file,data_file })
    }
    fn config(index_file:&'a str,data_file:&'a str) -> SKV<'a> {
        Self { index_file,data_file }
    }
    fn get(self,key:&'a str) -> std::io::Result<String> {
       // let file = std::fs::File::open(self.index_file)?;
        println!("{}",key);
        let str1 = std::fs::read_to_string(self.data_file)?;
        Ok(str1)
    }
}