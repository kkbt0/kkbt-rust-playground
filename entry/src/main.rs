use kkbt::SKV;
fn main() {
    // SKV::init("data/index", "data/data").unwrap();

    let skv = SKV::config("data/index", "data/data");

    skv.set("kkbt", Vec::from("我是恐咖兵糖1".as_bytes()));
    skv.set("kkbt", Vec::from("".as_bytes()));
    assert_eq!(skv.get_utf8("kkbt"),None);

    skv.set("kkbt", Vec::from("我是恐咖兵糖3".as_bytes()));
    assert_eq!(skv.get_utf8("kkbt"),Some("我是恐咖兵糖3".to_string()));

    assert_eq!(skv.get_utf8("abcdefg"),None);

   // kkbt::show_file_hex("data/index", 3).unwrap();
}
