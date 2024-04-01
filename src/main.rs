use std::thread;
use std::time::{Duration, Instant};
use std::io::{self, Write};
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

fn get_i64() -> i64 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    if input_text.trim().is_empty() {return 0;};
    let trimmed = input_text.trim();
    trimmed.parse::<i64>().unwrap()
}

fn wait() {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
}

fn play_beep() {
    play::play("beep-02.mp3").unwrap();
    //thread::sleep(Duration::from_millis(60));
}

fn start_timer() {
    let fps = 59.715;//60;
    //update fps? 59,72603066382307
    let count_down_times = 20;
    println!("Enter wanted frame:");
    let mut desired_frame = get_i64();
    println!("Enter delay");
    let mut delay = get_i64();
    let mut first = true;
    loop {
        if !first {
            println!("Update desired frame? Enter to skip");
            let new_frame = get_i64();
            if new_frame != 0 {desired_frame = new_frame}
        }
        let target_frame = desired_frame - delay;
        // Update desired frame
        first = false;
        
        let millisecs = ((target_frame) * 1000) as f64 / fps;
        let duration = Duration::from_millis(millisecs as u64);
        let count_down_time = Duration::from_millis(500 * count_down_times);
        
        println!("Countdown timer for {:?} ...", duration);
        println!("Press enter to start");
        wait();
    
        let start_time = Instant::now();
    
        while start_time.elapsed() < duration-count_down_time {
            let remaining_time = duration - start_time.elapsed();
            print!(
                "\rTime remaining: {} seconds",
                remaining_time.as_secs()
            );

            io::stdout().flush().expect("Failed to flush stdout");
            thread::sleep(Duration::from_millis(10));
        }
        
        // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // // Load a sound from a file, using a path relative to Cargo.toml
        // let file = BufReader::new(File::open("beep-02.mp3").unwrap());
        // // Decode that sound file into a source
        // let source = Decoder::new(file).unwrap();
        // // Play the sound directly on the device
        // let res = stream_handle.play_raw(source.convert_samples());
        play_beep();
        

        println!();
        println!("press in:");
        let mut last = (duration - start_time.elapsed()).as_millis();
        while start_time.elapsed() < duration - Duration::from_millis(20) {
            let remaining_time = duration - start_time.elapsed();
            let remaining_millis = remaining_time.as_millis();
            let count_down = last/500;
            if remaining_millis % 500 > last % 500 {
                println!("{}", count_down);
            }
    
            last = remaining_millis;
        }
    
        println!("now!");
        
        thread::sleep(Duration::from_secs(1));
        println!("What frame did you get?");
        let frame_got = get_i64();
        if frame_got == desired_frame {break};
        if frame_got == 0 {continue}
        let difference = desired_frame - frame_got;
        delay -= difference;

        println!("Delay updated to {}", delay);
    }

}

fn main() {
    start_timer();
}