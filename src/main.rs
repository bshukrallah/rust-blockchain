use blockchainlib::*;


fn main() {

    let block = Block::new(0, current_time(), vec![0; 32], 0, "Genesis Block".to_owned());


    println!("{:?}", &block);

}
