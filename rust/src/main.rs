mod import;

fn main() {
    let file_name = "C:\\Users\\shiwi\\GitHub\\digit-classification\\samples\\train.csv";

    println!("Hello, world!");
    import::import_csv(file_name).map_err(|err| println!("{:?}", err)).ok();
}
