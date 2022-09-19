mod lib;

fn main() {

    let result = lib::parse("/data/sample.srt");
    println!("Parsed SRT: {:?}", result);

    //lib::write_data(result, "/data/output.csv", true).unwrap();

}