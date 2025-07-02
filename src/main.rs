use specdb::get_spec_db;


fn main() {
    let spec_db = get_spec_db("/home/smear/personal/SpecDB/specs".to_string());
    
    println!("Files parsed total: {}", spec_db.files.iter().count());
}
// jt_lib = {path = "/home/camascounty/programming/rust/mine/jt_lib"}
// ^ Cargo.toml
// split the lib into different repository