use blocks::SEPARATOR;
use xcb::{Xid, x};

use std::{
    io::Error,
    rc::{Rc, Weak},
    sync::Arc,
    sync::Mutex,
    sync::atomic::{AtomicBool, Ordering},
    thread, time,
};
use subprocess::{CaptureData, Exec, ExitStatus, Popen, PopenError, Redirection};
mod blocks;
use signal_hook::flag;
//mod threads;

// DSR = Display Screen Root
struct DSR {
    conn: xcb::Connection,
    screen_num: i32,
    root_window: x::Window,
}
impl DSR {
    pub fn new<'a>() -> Result<DSR, xcb::ConnError> {
        // let (conn, screen_num) = xcb::Connection::connect(None)?;
        let (conn, screen_num) = xcb::Connection::connect(Some(":0"))?;

        let root = {
            let setup = conn.get_setup();
            let screen = setup.roots().nth(screen_num as usize).unwrap();
            screen.root()
        };

        let mut xconn = DSR {
            conn: conn,
            screen_num,

            root_window: root,
        };
        Ok(xconn)
    }
}

/* status line struct with the actual value for output
* and the last value to check if paint or not */
struct StatusLine {
    actual_status: String,
    last_status: String,
}
impl StatusLine {
    pub fn new() -> StatusLine {
        StatusLine {
            actual_status: String::new(),
            last_status: String::new(),
        }
    }
    pub fn not_equal(&self) -> bool {
        self.actual_status != self.last_status
    }
}

fn getcmd(block: &blocks::Block) -> String {
    let mut tmpstatus: String = String::new();
    tmpstatus.push_str(block.icon);
    let cmd_output: Result<CaptureData, PopenError> = Exec::shell(block.command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe)
        .capture();
    match cmd_output {
        Ok(output) => {
            let mut result = output.stdout_str();
            result = result.trim().to_string();
            tmpstatus.push_str(result.as_str());
        }
        Err(error) => {
            println!("Error {}, Command: {} ", error, &block.command);
            tmpstatus.push_str(String::from("error").as_str())
        }
    }
    tmpstatus.to_string()
}
fn dummy_sig_handler() {}

fn getcmds(counter: u128, cmd_results: &mut Vec<String>) {
    let blocks_size = blocks::BLOCKS.len();
    let results = Arc::new(Mutex::new(vec![String::new(); blocks_size]));

    let mut handles = vec![];
    for i in 0..blocks_size {
        let results_clone = Arc::clone(&results);
        // only run the command if it's time for it
        if (blocks::BLOCKS[i].interval != 0) && (counter % blocks::BLOCKS[i].interval as u128 == 0)
        {
            let handle = thread::spawn(move || {
                let mut result = results_clone.lock().unwrap();
                result[i] = getcmd(&blocks::BLOCKS[i])
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
    for idx in 0..blocks_size {
        let resultarray = results.lock().unwrap();
        if !resultarray[idx].is_empty() && resultarray[idx] != cmd_results[idx] {
            println!("rewrite: {}", resultarray[idx].clone());
            cmd_results[idx] = resultarray[idx].clone();
        }
    }
}

fn status_loop(x_attributes: DSR, sig_term: Arc<AtomicBool>) {
    let duration = time::Duration::from_millis(1000);
    let mut status_line = StatusLine::new();
    let mut times: u128 = 0;
    let mut cmd_results: Vec<String> = vec![String::new(); blocks::BLOCKS.len()];

    // loops as long as no signal has been given
    while !sig_term.load(Ordering::Relaxed) {
        getcmds(times, &mut cmd_results);
        writestatus(&x_attributes, &mut cmd_results, &mut status_line);
        thread::sleep(duration);
        times += 1;
    }
}

// write status to root window
fn writestatus(x_attributes: &DSR, cmd_results: &mut Vec<String>, status_line: &mut StatusLine) {
    // only if actually something differs
    if getstatus(cmd_results, status_line) {
        let root_window = x_attributes.root_window;
        let title_str = status_line.actual_status.clone();

        let cookie = x_attributes.conn.send_request_checked(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: root_window,
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: &title_str.as_bytes(),
        });
        let _ = x_attributes.conn.flush();
    }
}

// create status_line and return true if actual and laste are not equal
fn getstatus(cmd_results: &mut Vec<String>, status_line: &mut StatusLine) -> bool {
    status_line.last_status = status_line.actual_status.to_string().clone();
    // reset actual status so nothing adds up
    status_line.actual_status = String::new();
    for cmd_result in &mut cmd_results.clone().iter() {
        // we need to check if the const is empty because of
        // SEPARATOR is part of the configuration → this is intentional
        let mut local_result = cmd_result.clone();
        if !SEPARATOR.is_empty() {
            local_result.push_str(SEPARATOR);
        }

        status_line.actual_status.push_str(&local_result)
    }
    status_line
        .actual_status
        // .truncate(status_line.actual_status.len() - SEPARATOR.len());
        .truncate(status_line.actual_status.len());

    status_line.not_equal()
}

fn main() -> Result<(), Error> {
    let x_attributes = DSR::new().unwrap();

    // Declare bool, setting it to false
    let sig_term = Arc::new(AtomicBool::new(false));

    // Ask signal_hook to set the term variable to true
    // when the program receives a SIGTERM SIGINT signal
    flag::register(signal_hook::consts::SIGTERM, Arc::clone(&sig_term))?;
    flag::register(signal_hook::consts::SIGINT, Arc::clone(&sig_term))?;
    status_loop(x_attributes, Arc::clone(&sig_term));
    Ok(())
}
