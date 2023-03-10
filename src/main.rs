use getch::Getch;
use std::{io, env};
use std::path::Path;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use rand::{thread_rng, Rng};
use clearscreen::ClearScreen;
use winput::{press, release, send, Vk};
use winapi::um::winuser::{GetAsyncKeyState, VkKeyScanW};
use savefile::prelude::*;
#[macro_use]
extern crate savefile_derive;


fn main() {

    //setting up the standad values used
    let path = env::var("USERPROFILE");
    let saving_path = format!("{}\\Documents\\autoscan.svf", path.unwrap());
    let mut dkey:char = 'v';
    let mut mint:u8 = 3;
    let mut maxt:u8 = 6;
    
    //if a save file exists replace the standad values with the saved values
    if Path::new(&saving_path).exists() {
        #[derive(Savefile)]
        struct Man{key: String, min: u8, max: u8}
        let reload:Man = load_file(&saving_path, 0).unwrap();
        mint = reload.min;
        maxt = reload.max;
        dkey = reload.key.chars().next().unwrap();
    }
    let mut motd = "";
    
    //starting the actual menu
    loop {
        cls();
        settext(dkey, mint, maxt, motd);
        let getch = Getch::new();
        let raw = getch.getch().unwrap();
        let usr = raw as char;
        motd = "";
        if usr == 's'{
            cls();
            println!("{}", START);
            let vssk = unsafe {VkKeyScanW(dkey as u16)};
            dtool(dkey, mint, maxt, vssk.try_into().unwrap());
            motd = PAUSE;
            
            
            //change the min value
        } else if usr == 'n'{
            println!("{}", LOW);
            mint = match input().parse(){
                Ok(n) => n,
                Err(..)=>{
                    motd = INVALIDE;
                    continue;
                }
            };
            match save_to_file(dkey, maxt, mint, &saving_path){
                Ok(()) => (),
                Err(..)=> {
                    motd = FAIL_TO_SAVE;
                    continue;
                }
            };
            

            // Change the max value
        } else if usr == 'm'{
            println!("{}", HIGH);
            maxt = match input().parse(){
                Ok(n) => n,
                Err(..)=>{
                     motd = INVALIDE;
                    continue;
                }
            };
            match save_to_file(dkey, maxt, mint, &saving_path){
                Ok(()) => (),
                Err(..)=> {
                    motd = FAIL_TO_SAVE;
                    continue;
                }
            };
            

            // Change the key to d-scan
        } else if usr == 'k'{
            println!("{}", KEY_TXT);
            dkey = match input().parse(){
                Ok(n) => n,
                Err(..)=> {
                    motd = INVALIDE;
                    continue;
                }
            };
            match save_to_file(dkey, maxt, mint, &saving_path){
                Ok(()) => (),
                Err(..)=> {
                    motd = FAIL_TO_SAVE;
                    continue;
                }
            };
            

            //Quit the Program
        } else if usr == 'q'{
            cls();
            println!("Program Ended");
            exit(0);
        } else{
            motd = INVALIDE;
        }
    }
}


fn cls(){
    ClearScreen::default().clear().expect("faild");
}


fn dtool(k:char, n:u8, m:u8, v:u16){
    let mut count = 1;
    let mut fix_m = m as f32;
    let mut fix_n = n as f32;
    if n > m {
        fix_m = n as f32;
        fix_n = m as f32;
                
    } else if n == m {
        fix_m = m as f32 + 0.001;
        fix_n = n as f32;
    } loop {
        let wait_time = thread_rng().gen_range(fix_n..fix_m);
        let press_time = thread_rng().gen_range(80..170);
        press(k);
        sleep(Duration::from_millis(press_time));
        release(k);
        send(Vk::Backspace);
        sleep(Duration::from_secs_f32(wait_time));
        let mut p_pressed = unsafe {GetAsyncKeyState(v as i32)};
        if count < 2{
            p_pressed = 0;
        }
        if p_pressed != 0 {
            break;
        } else {
            count_form(count, wait_time);
            count += 1;
        }  
    }
}


fn input() -> String{
    loop{

        let mut inp = String::new();
        io::stdin().read_line(&mut inp).expect("faild to read line");
        let inp: String = match inp.trim().parse(){
            Ok(num) => num,
            Err(..) => continue,
        };
        return inp.trim().to_lowercase()
    }
}


fn save_to_file(k:char, m:u8, n:u8, p:&str) -> Result<(), SavefileError> {

    #[derive(Savefile)]
    struct Man{key: String, min: u8, max: u8}
    let tup = Man {key: k.to_string(), max: m, min: n};
    save_file(p ,0, &tup)?;
    Ok(())
}


//the place to mod the text output
fn settext(k:char, n:u8, m:u8, o:&str){
    println!("
D-Scan Tool Menu
-----------------
Start auto Scanning     --  <S>
Quit Program            --  <Q>

Set Default D-Scan key  --  <K> ({})
Set Minimal Scan Time   --  <N> ({})
Set Maximal Scan Time   --  <M> ({})

{}
", k, n, m, o);
}

fn count_form(c:i32, t:f32) {
    println!("{}. {:.2} sec.",c , t);
}

static PAUSE:&str = "Auto Scan Paused";
static INVALIDE:&str = "Invalid input";
static FAIL_TO_SAVE:&str = "Faild to save to file";
static KEY_TXT:&str = "Enter the Key you are useing to D-Scan";
static LOW:&str = "Enter the minimal time in seconds to Scan\n0-255";
static HIGH:&str = "Enter the maximal time in seconds to Scan\n0-255";
static START:&str = "D-Scan Tool started\nD-Scan manual to return to the Menu\n";
