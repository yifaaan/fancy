use super::{Height, Width};
use std::os::unix::io::AsFd;

pub fn terminal_size() -> Option<(Width, Height)> {
    if let Some(size) = terminal_size_of(std::io::stdout()) {
        Some(size)
    } else if let Some(size) = terminal_size_of(std::io::stderr()) {
        Some(size)
    } else if let Some(size) = terminal_size_of(std::io::stdin()) {
        Some(size)
    } else {
        None
    }
}

/// Returns the size of the terminal.
fn terminal_size_of<Fd: AsFd>(fd: Fd) -> Option<(Width, Height)> {
    use rustix::termios::{isatty, tcgetwinsize};

    // 是否指向终端terminal
    if !isatty(&fd) {
        return None;
    }

    let winsize = tcgetwinsize(&fd).ok()?;
    let rows = winsize.ws_row;
    let cols = winsize.ws_col;

    if rows > 0 && cols > 0 {
        Some((Width(cols), Height(rows)))
    } else {
        None
    }
}

// width = 2, height = 3,

#[test]
fn compare_with_stty() {
    use std::process::{Command, Stdio};

    let (rows, cols) = if cfg!(target_os = "illumos") {
        // 子进程执行stty命令

        let output = Command::new("stty")
            .stdin(Stdio::inherit())
            .output()
            .unwrap();
        assert!(output.status.success());
        // rows = 24; columns = 80; line = 0; ...
        let vals = String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .map(|line| {
                line.split(";")
                    .map(str::trim)
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .flatten() // -> Vec<String>
            .filter_map(|term| {
                let ss = term.splitn(2, " = ").collect::<Vec<_>>();

                match ss.as_slice() {
                    ["rows", n] | ["columns", n] => Some(n.parse::<u16>().unwrap()),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();
        (vals[0], vals[1])
    } else {
        let output = if cfg!(target_os = "linux") {
            // stty size -F /dev/stderr
            // 28 108

            Command::new("stty")
                .args(["size", "-F", "/dev/stderr"])
                .stderr(Stdio::inherit())
                .output()
                .unwrap()
        } else {
            Command::new("stty")
                .args(["-f", "/dev/stderr", "size"])
                .stderr(Stdio::inherit())
                .output()
                .unwrap()
        };
        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        let data = stdout.split_whitespace();
        println!("{}", stdout);
        let vals = data.map(|s| s.parse::<u16>().unwrap()).collect::<Vec<_>>();
        (vals[0], vals[1])
    };
    println!("{} {}", rows, cols);

    if let Some((Width(w), Height(h))) = terminal_size() {
        assert_eq!(rows, h);
        assert_eq!(cols, w);
    } else {
        panic!("terminal_size() return None");
    }
}
