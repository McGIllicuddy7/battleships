use std::{
    collections::VecDeque,
    io::Write,
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Clone)]
pub struct IO {
    inner: Arc<Mutex<IoInner>>,
}

#[allow(unused)]
struct IoInner {
    stdin: std::io::StdinLock<'static>,
    stderr: std::io::StderrLock<'static>,
    stdout: std::io::StdoutLock<'static>,
}

impl IO {
    #[allow(unused)]
    fn take_lock<'a>(&'a self) -> MutexGuard<'a, IoInner> {
        self.inner.lock().unwrap()
    }

    pub fn write_file(
        &self,
        path: impl AsRef<std::path::Path>,
        contents: impl AsRef<[u8]>,
    ) -> Result<(), std::io::Error> {
        std::fs::write(path, contents)
    }

    pub fn read_file(&self, path: impl AsRef<std::path::Path>) -> Result<Vec<u8>, std::io::Error> {
        std::fs::read(path)
    }

    pub fn write(&self, text: impl AsRef<[u8]>) {
        self.inner
            .lock()
            .unwrap()
            .stdout
            .write(text.as_ref())
            .unwrap();
    }

    pub fn err_write(&self, text: impl AsRef<[u8]>) {
        self.inner
            .lock()
            .unwrap()
            .stderr
            .write(text.as_ref())
            .unwrap();
    }

    pub fn get_line(&self, buf: &mut String) -> usize {
        use std::io::BufRead;
        self.inner.lock().unwrap().stdin.read_line(buf).unwrap()
    }
}
