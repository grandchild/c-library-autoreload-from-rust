use std::fs::metadata;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

// libloading
/*
use libloading::{Library, Symbol};

struct Lib {
    file_path: String,
    lib: Library,
}

impl Lib {
    fn _new(file_path: &str) -> Result<Self, libloading::Error> {
        let ret = Self {
            file_path: file_path.to_string(),
            lib: unsafe { Library::new(file_path) }?,
        };
        Ok(ret)
    }

    pub fn new(file_path: &str) -> Result<Self, String> {
        Self::_new(file_path).map_err(|e| format!("{:?}", e))
    }

    fn test(&self) {
        unsafe {
            let foo: Symbol<unsafe extern "C" fn() -> i32> = self.lib.get(b"foo").unwrap();
            println!("foo() -> {:?}", foo());
            let bar: Option<Symbol<unsafe extern "C" fn() -> i32>> = self.lib.get(b"bar").ok();
            if let Some(bar) = bar {
                println!("bar() -> {:?}", bar());
            } else {
                println!("bar() is not defined");
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
    file_path: String,
    api: Container<Api>,
}

impl Lib {
    fn _new(file_path: &str) -> Result<Self, dlopen2::Error> {
        let api_container: Container<Api> = unsafe { Container::load(file_path) }?;
        Ok(Self {
            file_path: file_path.to_string(),
            api: api_container,
        })
    }

    fn new(file_path: &str) -> Result<Self, String> {
        Self::_new(file_path).map_err(|e| format!("{:?}", e))
    }

    fn test(&self) {
        println!("foo() -> {:?}", unsafe { self.api.foo() });
        if let Some(bar) = self.api.bar {
            println!("bar() -> {:?}", unsafe { bar() });
        } else {
            println!("bar() is not defined");
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
    file_path: String,
    lib: Library,
}

impl Lib {
    fn _new(file_path: &str) -> Result<Self, dlopen2::Error> {
        let ret = Self {
            file_path: file_path.to_string(),
            lib: Library::open(file_path)?,
        };
        Ok(ret)
    }
    pub fn new(file_path: &str) -> Result<Self, String> {
        Self::_new(file_path).map_err(|e| format!("{:?}", e))
    }

    fn test(&self) {
        let api = unsafe { Api::load(&self.lib) }.unwrap();
        println!("foo() -> {:?}", unsafe { (api.foo)() });
        if let Some(bar) = api.bar {
            println!("bar() -> {:?}", unsafe { bar() });
        } else {
            println!("bar() is not defined");
        }
    }
}

// */
fn main() -> Result<(), String> {
    let mut last_mtime = SystemTime::UNIX_EPOCH;
    loop {
        let lib = Lib::new("build/libfoo.so")?;
        loop {
            lib.test();
            sleep(Duration::from_secs(1));

            if let Ok(file_metadata) = metadata(lib.file_path.as_str()) {
                if let Ok(mtime) = file_metadata.modified() {
                    if last_mtime != SystemTime::UNIX_EPOCH && last_mtime != mtime {
                        println!("Reloading {:?}", lib.file_path);
                        last_mtime = mtime;
                        break;
                    }
                    last_mtime = mtime;
                }
            }
        }
    }
}
