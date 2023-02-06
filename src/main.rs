use console::style;
use indicatif::{ProgressBar,  style::ProgressStyle};
use std::{time, thread};
use clap::Parser;

#[derive(Parser)]
struct Arguments{
    #[arg(short='r',long="rounds", default_value_t = 1 )]
    rounds : u32,


    #[arg(short='w',long="work", default_value_t = 45 )]
    work_time : u64,


    #[arg(short='x',long="relax", default_value_t = 15 )]
    relax_time : u64,
}

fn main() {
    
    let args = Arguments::parse();

    //greetings and info about rounds and timing
    println!("{}\n",
             style("work - a pomodoro-CLI").green());

    //enter the work loop
    for i in 0..args.rounds*2 {
        let bar : ProgressBar;
        if i % 2 == 0 { //work time
            bar = create_progress_bar("blue", args.work_time, "left to work");
        }
        else { //relax time
            bar = create_progress_bar("yellow", args.relax_time, "left to relax");
        }
        ProgressBar::set_message(&bar, format!("Phase {}/{}", (i+1), args.rounds*2));
        loop_through_bar(&bar);
        ProgressBar::finish(&bar);
    }

}

fn loop_through_bar(bar : &ProgressBar){
    let len = ProgressBar::length(bar);
    
    for _ in 0..len.unwrap(){
        thread::sleep(time::Duration::from_secs(1));
        bar.inc(1);
    }

}

fn create_progress_bar(color : &str, duration : u64, message : &str) -> ProgressBar {
    let pb = ProgressBar::new(duration*60);
    let template = format!("{{msg}} | {{bar:40.{}}} | {{eta}} {}", color, message);
    pb.set_style(ProgressStyle::with_template(&template)
                 .unwrap());
    return pb;
}


