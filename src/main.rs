use blocks::SEPARATOR;
use xcb::{Xid, x};

use std::{
    io::Error,
    sync::Arc,
    sync::atomic::{AtomicBool, Ordering},
    thread, time,
};
use subprocess::{CaptureData, Exec, ExitStatus, Popen, PopenError, Redirection};
mod blocks;
use signal_hook::flag;

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
    let cmd_output: Result<CaptureData, PopenError> = Exec::shell(&block.command)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Pipe)
        .capture();
    match cmd_output {
        Ok(output) => {
            let mut result = output.stdout_str();
            result = result.trim().to_string();
            // we need to check if the const is empty because of
            // SEPARATOR is part of the configuration → this is intentional
            if !SEPARATOR.is_empty() {
                result += SEPARATOR;
            }
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

fn getcmds() -> Vec<String> {
    let mut results: Vec<String> = vec![];
    for block in blocks::BLOCKS {
        results.push(getcmd(block));
    }
    results
}

fn status_loop(x_attributes: DSR, sig_term: Arc<AtomicBool>) {
    // external update signals turned of for now
    // setup_signals();
    let duration = time::Duration::from_millis(1000);
    let mut status_line = StatusLine::new();
    getcmds();
    // loops as long as no signal has been given
    while !sig_term.load(Ordering::Relaxed) {
        let cmd_results = getcmds();
        writestatus(&x_attributes, cmd_results, &mut status_line);
        thread::sleep(duration);
    }
}

fn writestatus(x_attributes: &DSR, cmd_results: Vec<String>, status_line: &mut StatusLine) {
    if getstatus(cmd_results, status_line) {
        let root_window = x_attributes.root_window;
        let title_str = status_line.actual_status.clone();

        println!("{}", title_str);

        let cookie = x_attributes.conn.send_request_checked(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: root_window,
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: &title_str.as_bytes(),
        });
        x_attributes.conn.flush();
    }
}

fn getstatus(cmd_results: Vec<String>, status_line: &mut StatusLine) -> bool {
    // println!("first_last: {}\n", status_line.actual_status);
    status_line.last_status = status_line.actual_status.to_string().clone();
    // reset actual status so nothing adds up
    status_line.actual_status = String::new();
    for cmd_result in cmd_results.iter() {
        status_line.actual_status.push_str(cmd_result)
    }
    status_line
        .actual_status
        .truncate(status_line.actual_status.len() - SEPARATOR.len());
    status_line.not_equal()
}

// this should somehow stop the main loop
fn sig_term() {}

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
