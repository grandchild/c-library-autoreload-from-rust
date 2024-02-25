use std::fs::metadata;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

// libloading
/*
use libloading::{Library, Symbol};

struct Lib {
    i: u32,
    file_path: String,
    lib: Library,
}

impl Lib {
    fn _new(file_path: &str, i: u32) -> Result<Self, libloading::Error> {
        let ret = Self {
            i,
            file_path: file_path.to_string(),
            lib: unsafe { Library::new(file_path) }?,
        };
        Ok(ret)
    }

    pub fn new(file_path: &str, i: u32) -> Result<Self, String> {
        Self::_new(file_path, i).map_err(|e| format!("{:?}", e))
    }

    fn test(&self) {
        unsafe {
            let foo: Symbol<unsafe extern "C" fn() -> i32> = self.lib.get(b"foo").unwrap();
            println!("[{}] foo() -> {:?}", self.i, foo());
            let bar: Option<Symbol<unsafe extern "C" fn() -> i32>> = self.lib.get(b"bar").ok();
            if let Some(bar) = bar {
                println!("[{}] bar() -> {:?}", self.i, bar());
            } else {
                println!("[{}] bar() is not defined", self.i);
            }
        }
    }
}

// */

// dlopen2 - wrapper api
//*
use dlopen2::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct Api {
    foo: unsafe extern "C" fn() -> i32,
    bar: Option<unsafe extern "C" fn() -> i32>,
}

struct Lib {
    i: u32,
    file_path: String,
    api: Container<Api>,
}

impl Lib {
    fn new(file_path: &str, i: u32) -> Result<Self, String> {
        let api_container: Container<Api> =
            unsafe { Container::load(file_path) }.map_err(|e| format!("{:?}", e))?;
        Ok(Self {
            i,
            file_path: file_path.to_string(),
            api: api_container,
        })
    }

    fn test(&self) {
        println!("[{}] foo() -> {:?}", self.i, unsafe { self.api.foo() });
        if let Some(bar) = self.api.bar {
            println!("[{}] bar() -> {:?}", self.i, unsafe { bar() });
        } else {
            println!("[{}] bar() is not defined", self.i);
        }
    }
}

// */
// dlopen2 - symbor api
/*
use dlopen2::symbor::{Library, SymBorApi, Symbol};

#[derive(SymBorApi)]
struct Api<'a> {
    pub foo: Symbol<'a, unsafe extern "C" fn() -> i32>,
    pub bar: Option<Symbol<'a, unsafe extern "C" fn() -> i32>>,
}

struct Lib {
    i: u32,
    file_path: String,
    lib: Library,
}

impl Lib {
    fn _new(file_path: &str, i: u32) -> Result<Self, dlopen2::Error> {
        let ret = Self {
            i,
            file_path: file_path.to_string(),
            lib: Library::open(file_path)?,
        };
        Ok(ret)
    }
    pub fn new(file_path: &str, i: u32) -> Result<Self, String> {
        Self::_new(file_path, i).map_err(|e| format!("{:?}", e))
    }

    fn test(&self) {
        let api = unsafe { Api::load(&self.lib) }.unwrap();
        println!("[{}] foo() -> {:?}", self.i, unsafe { (api.foo)() });
        if let Some(bar) = api.bar {
            println!("[{}] bar() -> {:?}", self.i, unsafe { bar() });
        } else {
            println!("[{}] bar() is not defined", self.i);
        }
    }
}

// */

fn main() -> Result<(), String> {
    let mut i = 0u32;
    let mut last_mtime = SystemTime::UNIX_EPOCH;
    loop {
        let lib = Lib::new("build/libfoo.so", i)?;
        loop {
            lib.test();
            sleep(Duration::from_secs(1));

            if let Ok(file_metadata) = metadata(lib.file_path.as_str()) {
                if let Ok(mtime) = file_metadata.modified() {
                    if last_mtime != SystemTime::UNIX_EPOCH && last_mtime != mtime {
                        println!("Reloading {:?}", lib.file_path);
                        i += 1;
                        last_mtime = mtime;
                        break;
                    }
                    last_mtime = mtime;
                }
            }
        }
    }
}
