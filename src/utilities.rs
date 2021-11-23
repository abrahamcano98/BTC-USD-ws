/* This file has the minimal functions to extract real time data from 
binance web socket api: "https://github.com/binance/binance-spot-api-docs/blob/master/web-socket-streams.md"*/




// We import the needed libraries
use tungstenite::connect;
use url::Url;
use std::time::Instant;
use serde_json;
use std::fs;
use std::io::{BufWriter, Write};
use itertools::Itertools; 

use crate::visualization;




//Basic struct for retrieving data from the websocket.
pub struct Sample {
    pub price: f32,
    pub quantity: f32,
  }



/*Function to establish a connection with the binance websocket and pick up the data
from it for "time_exec" seconds */
pub fn connection(path: &str,verbose: bool,time_exec: u64)->Vec<Sample>{
    static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

    let binance_str = format!(
        "{}/{}",
        BINANCE_WS_API,path
    );
    let binance_url=Url::parse(&binance_str).unwrap();
    let (mut socket, response) = connect(binance_url).expect("Failed to connect");
    //Log to identify wrong connections if necessary.
    if verbose{
        println!("Connected to binance stream.");
        println!("HTTP status code: {}", response.status());
        println!("Response headers:");
        for (ref header, ref header_value) in response.headers() {
            println!("- {}: {:?}", header, header_value);
        }
    }
    //Loop to retrieve the data from the websocket and store them into the vector sample_vec.
    
    let mut sample_vec =  std::vec::Vec::new();
    let now = Instant::now();
    println!("{}","Receiving data...");
    
    
    while  now.elapsed().as_secs() < time_exec  {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => { s }
            _ => { panic!() }
        };
        let parsed: serde_json::Value = serde_json::from_str(&msg).expect("Can't parse to JSON");
        let price_sample: f32=parsed["p"].as_str().unwrap().parse().unwrap();
        let quantity_sample: f32=parsed["q"].as_str().unwrap().parse().unwrap();
        sample_vec.push(Sample{price:price_sample, quantity:quantity_sample});
    }
    println!("{}","Cache Complete");
    sample_vec
}







/*Function to write the data received from the ws to a .txt file. If save_img==true, it returns a bar plot
of the aggregate  and the evolution of the BTC-USDT price during 10 seconds.*/
pub fn write_data(file_name: &str, input_data: Vec<Sample>, save_img: bool){
    fs::create_dir_all("output").expect("Not able to create a directory for results");
    let f = fs::File::create(format!("output/{}",file_name)).expect("unable to create file");
    let mut f = BufWriter::new(f);
    writeln!(f,"Raw trade stream \n \n").expect("Unable to write");
    writeln!(f,"BTC-USDT  \t\t  Quantity").expect("Unable to write");
    
    let mut aggregate=std::collections::HashMap::new();
    for sample in &input_data {
        writeln!(f, "{} \t \t {} ", format!("{:.2}",sample.price), format!("{:.5}",sample.quantity)).expect("Unable to write");
        aggregate.entry(format!("{:.2}",sample.price)).or_insert(0.0);
        aggregate.insert(format!("{:.2}",sample.price),sample.quantity+aggregate[&format!("{:.2}",sample.price)]);
    }

    
    writeln!(f,"\n \nAggregated volume prices \n \n").expect("Unable to write");
    writeln!(f,"BTC-USDT  \t\t  Quantity").expect("Unable to write");

    for key in aggregate.keys().sorted(){
        writeln!(f, "{} \t \t {}", key, format!("{:.5}",aggregate[key])).expect("Unable to write");
     }
     if save_img{
     visualization::time_series_plot(input_data).expect("Unable to generate the timeseries plot");
     visualization::bar_plot(aggregate);
     }
}


// Function to ppint file data to console.
pub fn show_data(){
    let data=fs::read_to_string("output/BTC-USD.txt").expect("No previous execution. Please, run \"cargo run cache\".");
    println!("{}",data)
    }
