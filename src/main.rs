mod utilities;
mod visualization;
use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len()!=2 {
         panic!("Invalid file arguments. Please, run either \"cargo run cache\" for retrieving the data from the socket or \"cargo run read\" to visualize the last execution");
    }
    let mode=&args[1];

    
    match mode.as_str(){
        "cache" => {
            let data=utilities::connection("ws/btcusdt@trade",false,10);
            utilities::write_data("BTC-USD.txt",data,true);
    },
        "read" =>{utilities::show_data()},
    
    _ => { panic!("Invalid file parameter. Please, run either \"cargo run cache\" for retrieving the data from the socket or \"cargo run read\" to visualize the last execution") },
}
}
