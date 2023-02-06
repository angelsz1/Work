use indicatif::{ProgressBar,  style::ProgressStyle};
use std::{time, thread};
use clap::Parser;

#[derive(Parser)]
struct Arguments{
    rounds : u32,
    work_time : u64,
    relax_time : u64,
}

fn main() {

    let args = Arguments::parse();
    //enter the work loop
    for i in 0..args.rounds*2 {
        let bar : ProgressBar;
        if i % 2 == 0 { //work time
            bar = create_progress_bar("blue", args.work_time, "Time to work");
        }
        else { //relax time
            bar = create_progress_bar("yellow", args.relax_time, "Time to relax");
        }
        ProgressBar::set_message(&bar, format!("Round number {}", (i/2 + 1)));
        loop_through_bar(&bar);
    }

}

fn loop_through_bar(bar : &ProgressBar){
    let len = ProgressBar::length(bar);
    
    for _ in 0..len.unwrap(){
        bar.inc(1);
        thread::sleep(time::Duration::from_secs(1));
    }

}

fn create_progress_bar(color : &str, duration : u64, message : &str) -> ProgressBar {
    let pb = ProgressBar::new(duration);
    let template = format!("{{msg}}\n[{{elapsed}}] {{bar:40.{}}} {}", color, message);
    pb.set_style(ProgressStyle::with_template(&template)
                 .unwrap());
    return pb;
}

