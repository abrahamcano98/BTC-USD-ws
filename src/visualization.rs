/*Use of visualization libraries, 'plotters' and 'plotlib'*/
use plotlib::page::Page;
use plotlib::repr::BarChart;
use plotlib::style::BoxStyle;
use plotlib::view::CategoricalView;
use plotters::prelude::*;

use std::collections::HashMap;
use std::iter::FromIterator;
use itertools::Itertools; 

use crate::core;


    /// Function to generate the time-series plot of the raw trade stream returned by the Binance websocket.
    ///
    ///
    /// Arguments:
    ///
    /// * `input_data`: Vector with a tuple (price,quantity) for each trade raw stram.
pub fn time_series_plot(input_data: Vec<core::Sample>) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut price_time=std::vec::Vec::new();
    let sample_len:f32=input_data.len() as f32;
    let sample_rate=10.0/sample_len;
    let mut t=0.0;
    let min_value:f32;
    let max_value:f32;

    for sample in &input_data {
        price_time.push((t*sample_rate,sample.price));
        t=t+1.0;
    }

    min_value=(*price_time.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap()).1;
    max_value=(*price_time.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap()).1;

    let root = BitMapBackend::new("output/BTC-USDT-timedata.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("BTC-USDT price", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .build_cartesian_2d(0f32..10f32,min_value..max_value)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
           price_time,
            &RED,
        ))?
        .label("BTC-USDT")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

/*Function to display a bar plot of the computed aggregate. It
masks the values with a window of size 8 due to technical issues
TODO: Try to show 'xlabel' on a horizontal way*/

    /// Function to display a bar plot of the computed aggregate. It
    /// masks the values with a window of size 8 due to technical issues
    ///TODO: Try to show 'xlabel' on a horizontal way.
    ///
    ///
    /// Arguments:
    ///
    /// * `aggregate`: Hashmaps with key equal to price and value equal to aggregated value of the price. 
pub fn bar_plot(aggregate:HashMap::<std::string::String, f32>){
    
    let mut aggregated_value:f32 =0.0;
    let  mut v = CategoricalView::new().x_label("BTC-USD");
    let mut label=Vec::from_iter(aggregate.keys())[0];
    let mut i=1;
    
    for key in aggregate.keys().sorted(){
        if i%8==0{
            let b=BarChart::new(aggregated_value.into()).label(label).style(&BoxStyle::new().fill("red"));
            v=v.add(b);
            label=key;
            aggregated_value=0.0;
        }
        aggregated_value=aggregated_value+aggregate[key];
        i=i+1;
}

    Page::single(&v).save("output/Aggregate-Bar.svg").expect("Not possible to save the file");
}