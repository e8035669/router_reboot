use chrono::Local;
use reqwest::blocking::Client;
use std::{thread, time::Duration};

pub struct Checker<'a> {
    client: Client,
    interval: u32,
    fail_times: u32,
    cooldown: u32,
    cur_fail: u32,
    on_failed: Box<dyn Fn() + 'a>,
}

impl<'a> Checker<'a> {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            interval: 60,
            fail_times: 5,
            cooldown: 300,
            cur_fail: 0,
            on_failed: Box::new(|| println!("Network failed!")),
        }
    }

    pub fn set_on_failed<F>(&mut self, func: F)
    where
        F: Fn() + 'a,
    {
        self.on_failed = Box::new(func);
    }

    pub fn start_check(&mut self) {
        loop {
            let res = self.client.get("https://www.google.com.tw/").send();
            let now = Local::now();
            let now = now.to_string();
            match res {
                Ok(_) => {
                    // println!("{}, network success", now);
                    self.cur_fail = 0;
                    // self.cur_fail += 1;
                }
                Err(msg) => {
                    println!("{}, network failed: {:?}", now, msg);
                    self.cur_fail += 1;
                }
            }
            if self.cur_fail >= self.fail_times {
                self.cur_fail = 0;
                self.on_failed.as_ref()();
                thread::sleep(Duration::from_secs(self.cooldown as u64));
            } else {
                thread::sleep(Duration::from_secs(self.interval as u64));
            }
        }
    }
}
