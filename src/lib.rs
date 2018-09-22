extern crate wasm_bindgen;

use std::fmt::Write;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);   
}

// from index.js
#[wasm_bindgen(module = "./index")]
extern {
    fn domain() -> Option<String>;
    fn random(n: i32) -> i32;
}


macro_rules! console_log {
    ($($t:tt)*) => {
        (log(&format_args!($($t)*).to_string()))
    };
}

/// test
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}", name));
    console_log!("greet() in Rust with arg: {}", name);
}

/// Page Conetnts
#[wasm_bindgen]
pub struct PageElems {
    title: String,
    body: String,
}

#[wasm_bindgen]
impl PageElems {
    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }
}

/// Main Page generator
#[wasm_bindgen]
pub fn home() -> PageElems {
    let name = domain().unwrap();

    let n = random(SIZE as i32) as usize;
    let (code, title) = CODES[n];
    let h1 = format!("{} {}", code, title);

    let port = random(10000);

    PageElems {
        title: h1.to_owned(),
        body: format!("<h1 style=\"text-decoration:none;color:black;text-align:center;\">{}</h1><br/><br/><br/><hr/><div style=\"font-style:italic;text-align:left\">{} Server at {} Port {}</div>", create_title(&h1), server(), name, port)
    }
}

/// Jump Page generator
#[wasm_bindgen]
pub fn jump() -> String {
    let n = random(JSIZE as i32) as usize;
    JUMPS[n].to_owned()
}

fn create_title(s: &str) -> String {
    let mut res = String::new();

    for c in s.chars() {
        write!(&mut res, "<a style=\"text-decoration:none;color:black;\" href=\"jump/\">{}</a>", c).unwrap();
    }

    res
}

fn server() -> String {
    let n = random(SSIZE as i32) as usize;
    let m = random(MSIZE as i32) as usize;
    let x = random(10);
    let y = random(10);
    let z = random(10);

    format!("{}/{}.{}.{} ({})", SERVERS[n], x, y, z, OS[m])
}

const MSIZE: usize = 35;

const OS: [&'static str; MSIZE] = [
    "Apple DOS",
    "Apple ProDOS",
    "Apple SOS",
    "Lisa Office System",
    "System 1",
    "System 2",
    "System 3",
    "System 4",
    "System 6",
    "System 7",
    "Mac OS 8",
    "Mac OS 9",
    "Newton OS",
    "MkLinux",
    "Mac OS X",
    "macOS",
    "iOS",
    "MS-DOS",
    "MSX-DOS",
    "Xenix",
    "Windows 95",
    "Windows 98",
    "Windows Millennium Edition",
    "Windows 2000",
    "Windows XP",
    "Windows Vista",
    "Singularity",
    "Midori",
    "iRMX",
    "Amiga Unix",
    "Android",
    "Google Chrome OS",
    "Steam OS",
    "Raspbian",
    "Fedora",
];

const SSIZE: usize = 13;

const SERVERS: [&'static str; SSIZE] = [
    "Apache",
    "nginx",
    "Internet Information Services",
    "AN HTTPD",
    "04WebServer",
    "lighttpd",
    "CERN HTTPd",
    "NCSA HTTPd",
    "Cherokee",
    "Hiawatha",
    "thttpd",
    "RaidenHTTPD",
    "Warp"
];

const JSIZE: usize = 13;

const JUMPS: [&'static str; JSIZE] = [
    "http://ippei-kun.com",
    "http://youtu.be/OxXzOA784X8?autoplay=1",
    "https://www.haskell.org/",
    "http://youtu.be/80_SkcqbaGk?autoplay=1",
    "http://youtu.be/_GXak5MSSLg?autoplay=1",
    "http://youtu.be/W5hpWjZ_HpU?autoplay=1",
    "http://youtu.be/IJwuSJlnTNA?autoplay=1", 
    "http://youtu.be/W5hpWjZ_HpU?autoplay=1",
    "http://youtu.be/cokDlOUWtyU?autoplay=1",
    "http://youtu.be/ICD9PSeweMk?autoplay=1",
    "http://youtu.be/gZlnahqfN2o?autoplay=1",
    "https://youtu.be/bp2ZOX-3F_I?autoplay=1",
    "https://www.rust-lang.org/en-US/"
];

const SIZE: usize = 61;

const CODES: [(i32, &'static str); SIZE] = [
        (100, "Continue"),
        (101, "Switching Protocol"),
        (102, "Processing"),

        (200, "OK"),
        (201, "Created"),
        (202, "Accepted"),
        (203, "Non-Authoritative Information"),
        (204, "No Content"),
        (205, "Reset Content"),
        (206, "Partial Content"),
        (207, "Multi-Status"),
        (208, "Multi-Status"),
        (226, "IM Used"),

        (300, "Multiple Choice"),
        (301, "Moved Permanently"),
        (302, "Found"),
        (303, "See Other"),
        (304, "Not Modified"),
        (305, "Use Proxy"),
        (306, "unused"),
        (307, "Temporary Redirect"),
        (308, "Permanent Redirect"),

        (400, "Bad Request"),
        (401, "Unauthorized"),
        (402, "Payment Required"),
        (403, "Forbidden"),
        (404, "Not Found"),
        (405, "Method Not Allowed"),
        (406, "Not Acceptable"),
        (407, "Proxy Authentication Required"),
        (408, "Request Timeout"),
        (409, "Conflict"),
        (410, "Gone"),
        (411, "Length Required"),
        (412,  "Precondition Failed"),
        (413, "Payload Too Large"),
        (414, "URI Too Long"),
        (415, "Unsupported Media Type"),
        (416, "Requested Range Not Satisfiable"),
        (417, "Expectation Failed"),
        (418, "I'm a teapot"),
        (421, "Misdirected Request"),
        (422, "Unprocessable Entity"),
        (423, "Locked"),
        (424, "Failed Dependency"),
        (426, "Upgrade Required"),
        (428, "Precondition Required"),
        (429, "Too Many Requests"),
        (431, "Request Header Fields Too Large"),
        (451, "Unavailable For Legal Reasons"),
        
        (500, "Internal Server Error"),
        (501, "Not Implemented"),
        (502, "Bad Gateway"),
        (503, "Service Unavailable"),
        (504, "Gateway Timeout"),
        (505, "HTTP Version Not Supported"),
        (506, "Variant Also Negotiates"),
        (507, "Insufficient Storage"),
        (508, "Loop Detected"),
        (510, "Not Extended"),
        (511, "Network Authentication Required")
];
