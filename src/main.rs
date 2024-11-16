use blockchainlib::*;


fn main() {

    let mut block = Block::new(0, current_time(), vec![0; 32], 0, "Genesis Block".to_owned());

    println!("{:?}", &block);

    let h = block.hash();
    println!("{:?}", &h);
    block.hash = h;
    println!("{:?}", &block);

    let h = block.hash();
    println!("{:?}", &h);
}
