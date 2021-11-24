mod core;
mod visualization;
use std::env;


fn main(){
    let args: Vec<String> = env::args().collect();
    
    if args.len()!=2 {
         panic!("Invalid file parameter. Please, run either \"cargo run cache\" \\ \"./client cache\" for retrieving the data
          from the socket or \"cargo run read\" \\ \"./client read\" to visualize the last execution");
    }
    
    let mode=&args[1];

    match mode.as_str(){
        "cache" => {
            let data=core::ws_connection("ws/btcusdt@trade",false,10);
            core::write_data("BTC-USD.txt",data,false);
        },
        "read" =>{core::show_data()
        },
    
    _ => { panic!("Invalid file parameter. Please, run either \"cargo run cache \" \\ \"./client cache\" for retrieving the data 
    from the socket or \"cargo run read\" \\ \"./client read\" to visualize the last execution") },
    }
}
