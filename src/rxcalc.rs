// By r-Scratch-Compiler 2024-05-08T11:42:58.021Z 🍥 v1.1-dev-feat-不使用 lazy-static
use std::sync::Arc;
use std::sync::Mutex;
fn get_f64_string<T: AsRef<str>>(toget: T) -> f64 {
    let num: Result<f64, _> = toget.as_ref().parse();
    match num {
        Ok(parsed_num) => parsed_num,
        Err(_) => 0.0,
    }
}
fn string_to_boolean<T: AsRef<str>>(input: T) -> bool {
    let trimmed_input = input.as_ref().trim();
    if trimmed_input.is_empty()
        || trimmed_input == "0"
        || trimmed_input.eq_ignore_ascii_case("false")
    {
        false
    } else {
        true
    }
}
fn get_char_at<T: AsRef<str>>(s: T, index: f64) -> String {
    s.as_ref()
        .chars()
        .nth((index - 1.0) as usize)
        .map(|c| c.to_string())
        .unwrap_or_default()
}
fn repeat<T: AsRef<str>>(s: T, times: usize) -> String {
    let mut result = String::new();
    for _ in 0..times {
        result.push_str(s.as_ref());
    }
    result
}
struct Default {
    l_1: Mutex<Vec<String>>,
    vf_1_4: Mutex<f64>,
    vs_1_4: Mutex<String>,
    vf_1_5: Mutex<f64>,
    vs_1_5: Mutex<String>,
    vf_1_6: Mutex<f64>,
    vs_1_6: Mutex<String>,
    vf_1_7: Mutex<f64>,
    vs_1_7: Mutex<String>,
    vf_1_8: Mutex<f64>,
    vs_1_8: Mutex<String>,
    vf_1_9: Mutex<f64>,
    vs_1_9: Mutex<String>,
    vf_1_10: Mutex<f64>,
    vs_1_10: Mutex<String>,
    l_1_11: Mutex<Vec<String>>,
    l_1_12: Mutex<Vec<String>>,
    l_1_13: Mutex<Vec<String>>,
}
impl Default {
    fn new() -> Default {
        Default {
            l_1: Mutex::new(
                [
                    "==", ">=", "<=", "!=", ">", "<", "+", "-", "*", "/", "%", "^", "&&", "||",
                    "..", "(", ")",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            ),
            vf_1_4: Mutex::new(3.0),
            vs_1_4: Mutex::new((3).to_string()),
            vf_1_5: Mutex::new(17.0),
            vs_1_5: Mutex::new((17).to_string()),
            vf_1_6: Mutex::new(0.0),
            vs_1_6: Mutex::new((0).to_string()),
            vf_1_7: Mutex::new(0.0),
            vs_1_7: Mutex::new(("").to_string()),
            vf_1_8: Mutex::new(1.0),
            vs_1_8: Mutex::new((1).to_string()),
            vf_1_9: Mutex::new(0.0),
            vs_1_9: Mutex::new((0).to_string()),
            vf_1_10: Mutex::new(1.0),
            vs_1_10: Mutex::new((1).to_string()),
            l_1_11: Mutex::new(vec![]),
            l_1_12: Mutex::new(["", "", "", "0"].iter().map(|s| s.to_string()).collect()),
            l_1_13: Mutex::new(["1"].iter().map(|s| s.to_string()).collect()),
        }
    }
    fn get_1_num_8(&self) -> f64 {
        let globalvar = *self.vf_1_8.lock().unwrap();
        globalvar
    }
    fn proc1(&self, pm_14: &str, pm_15: &str, pm_16: &str) {
        *self.vf_1_8.lock().unwrap() = 0.0;
        *self.vs_1_8.lock().unwrap() = (0.0).to_string();
        if pm_15.contains(&"==") {
            if String::from(pm_14) == String::from(pm_16) {
                *self.vf_1_8.lock().unwrap() = 1.0;
                *self.vs_1_8.lock().unwrap() = (1.0).to_string();
            } else {
                *self.vf_1_8.lock().unwrap() = 0.0;
                *self.vs_1_8.lock().unwrap() = (0.0).to_string();
            };
        } else {
            if pm_15.contains(&"!=") {
                if !(String::from(pm_14) == String::from(pm_16)) {
                    *self.vf_1_8.lock().unwrap() = 1.0;
                    *self.vs_1_8.lock().unwrap() = (1.0).to_string();
                } else {
                    *self.vf_1_8.lock().unwrap() = 0.0;
                    *self.vs_1_8.lock().unwrap() = (0.0).to_string();
                };
            } else {
                if pm_15.contains(&"<") {
                    if get_f64_string(pm_14) < get_f64_string(pm_16) {
                        *self.vf_1_8.lock().unwrap() = 1.0;
                        *self.vs_1_8.lock().unwrap() = (1.0).to_string();
                    } else {
                        *self.vf_1_8.lock().unwrap() = 0.0;
                        *self.vs_1_8.lock().unwrap() = (0.0).to_string();
                    };
                } else {
                    if pm_15.contains(&">") {
                        if get_f64_string(pm_14) > get_f64_string(pm_16) {
                            *self.vf_1_8.lock().unwrap() = 1.0;
                            *self.vs_1_8.lock().unwrap() = (1.0).to_string();
                        } else {
                            *self.vf_1_8.lock().unwrap() = 0.0;
                            *self.vs_1_8.lock().unwrap() = (0.0).to_string();
                        };
                    } else {
                        if pm_15.contains(&"<=") {
                            if get_f64_string(pm_14) <= get_f64_string(pm_16) {
                                *self.vf_1_8.lock().unwrap() = 1.0;
                                *self.vs_1_8.lock().unwrap() = (1.0).to_string();
                            } else {
                                *self.vf_1_8.lock().unwrap() = 0.0;
                                *self.vs_1_8.lock().unwrap() = (0.0).to_string();
                            };
                        } else {
                            if pm_15.contains(&">=") {
                                if get_f64_string(pm_14) >= get_f64_string(pm_16) {
                                    *self.vf_1_8.lock().unwrap() = 1.0;
                                    *self.vs_1_8.lock().unwrap() = (1.0).to_string();
                                } else {
                                    *self.vf_1_8.lock().unwrap() = 0.0;
                                    *self.vs_1_8.lock().unwrap() = (0.0).to_string();
                                };
                            } else {
                                if pm_15.contains(&"+") {
                                    *self.vf_1_8.lock().unwrap() =
                                        get_f64_string(pm_14) + get_f64_string(pm_16);
                                    *self.vs_1_8.lock().unwrap() =
                                        (get_f64_string(pm_14) + get_f64_string(pm_16)).to_string();
                                } else {
                                    if pm_15.contains(&"/") {
                                        *self.vf_1_8.lock().unwrap() =
                                            get_f64_string(pm_14) / get_f64_string(pm_16);
                                        *self.vs_1_8.lock().unwrap() = (get_f64_string(pm_14)
                                            / get_f64_string(pm_16))
                                        .to_string();
                                    } else {
                                        if pm_15.contains(&"*") {
                                            *self.vf_1_8.lock().unwrap() =
                                                get_f64_string(pm_14) * get_f64_string(pm_16);
                                            *self.vs_1_8.lock().unwrap() = (get_f64_string(pm_14)
                                                * get_f64_string(pm_16))
                                            .to_string();
                                        } else {
                                            if pm_15.contains(&"%") {
                                                *self.vf_1_8.lock().unwrap() =
                                                    get_f64_string(pm_14) % get_f64_string(pm_16);
                                                *self.vs_1_8.lock().unwrap() =
                                                    (get_f64_string(pm_14) % get_f64_string(pm_16))
                                                        .to_string();
                                            } else {
                                                if pm_15.contains(&"||") {
                                                    *self.vf_1_8.lock().unwrap() = get_f64_string(
                                                        if string_to_boolean(pm_14)
                                                            || string_to_boolean(pm_16)
                                                        {
                                                            (1.0).to_string()
                                                        } else {
                                                            (0.0).to_string()
                                                        }
                                                        .clone(),
                                                    );
                                                    *self.vs_1_8.lock().unwrap() =
                                                        if string_to_boolean(pm_14)
                                                            || string_to_boolean(pm_16)
                                                        {
                                                            (1.0).to_string()
                                                        } else {
                                                            (0.0).to_string()
                                                        };
                                                } else {
                                                    if pm_15.contains(&"&&") {
                                                        *self.vf_1_8.lock().unwrap() =
                                                            get_f64_string(
                                                                if string_to_boolean(pm_14)
                                                                    && string_to_boolean(pm_16)
                                                                {
                                                                    (1.0).to_string()
                                                                } else {
                                                                    (0.0).to_string()
                                                                }
                                                                .clone(),
                                                            );
                                                        *self.vs_1_8.lock().unwrap() =
                                                            if string_to_boolean(pm_14)
                                                                && string_to_boolean(pm_16)
                                                            {
                                                                (1.0).to_string()
                                                            } else {
                                                                (0.0).to_string()
                                                            };
                                                    } else {
                                                        if pm_15.contains(&"..") {
                                                            *self.vf_1_8.lock().unwrap() =
                                                                get_f64_string(
                                                                    pm_14.to_owned()
                                                                        + &pm_16.to_owned().clone(),
                                                                );
                                                            *self.vs_1_8.lock().unwrap() = pm_14
                                                                .to_owned()
                                                                + &pm_16.to_owned();
                                                        } else {
                                                            if pm_15.contains(&"^") {
                                                                *self.vf_1_8.lock().unwrap() =
                                                                    get_f64_string(pm_14);
                                                                *self.vs_1_8.lock().unwrap() =
                                                                    String::from(pm_14);
                                                                let repeatto =
                                                                    (get_f64_string(pm_14) - 1.0)
                                                                        as usize;
                                                                for _ in 0..repeatto {
                                                                    *self.vf_1_8.lock().unwrap() =
                                                                        self.get_1_num_8()
                                                                            * get_f64_string(pm_14);
                                                                    *self.vs_1_8.lock().unwrap() =
                                                                        (self.get_1_num_8()
                                                                            * get_f64_string(
                                                                                pm_14,
                                                                            ))
                                                                        .to_string();
                                                                }
                                                            } else {
                                                                if (String::from(pm_15)
                                                                    == String::from(""))
                                                                    && (String::from(pm_16)
                                                                        == String::from(""))
                                                                {
                                                                    *self.vf_1_8.lock().unwrap() =
                                                                        get_f64_string(pm_14);
                                                                    *self.vs_1_8.lock().unwrap() =
                                                                        String::from(pm_14);
                                                                } else {
                                                                    *self.vf_1_8.lock().unwrap() =
                                                                        get_f64_string("Error");
                                                                    *self.vs_1_8.lock().unwrap() =
                                                                        String::from("Error");
                                                                };
                                                            };
                                                        };
                                                    };
                                                };
                                            };
                                        };
                                    };
                                };
                            };
                        };
                    };
                };
            };
        };
    }

    fn get_contains_13(&self, item: String) -> bool {
        self.l_1_13.lock().unwrap().contains(&item)
    }
    fn get_position_13(&self, value_to_find: &str) -> f64 {
        if let Some(position) = self
            .l_1_13
            .lock()
            .unwrap()
            .iter()
            .position(|x| x == &value_to_find)
        {
            position as f64 + 1.0
        } else {
            0.0
        }
    }
    fn get_item_13(&self, item: usize) -> String {
        if let Some(value) = self.l_1_13.lock().unwrap().get(item) {
            value.to_string()
        } else {
            String::new()
        }
    }
    fn get_1_num_10(&self) -> f64 {
        let globalvar = *self.vf_1_10.lock().unwrap();
        globalvar
    }
    fn get_1_str_8(&self) -> String {
        let globalvar = self.vs_1_8.lock().unwrap().clone();
        globalvar
    }
    fn proc2(&self) {
        while !(!(self.get_contains_13(String::from("*")))
            && !(self.get_contains_13(String::from("/"))))
        {
            if self.get_position_13("*") > self.get_position_13("/") {
                self.proc1(
                    &*(self.get_item_13(
                        if let Some(result) =
                            ((self.get_position_13("*") - 1.0) as usize).checked_sub(1)
                        {
                            result
                        } else {
                            0
                        },
                    )),
                    &*(self.get_item_13(
                        if let Some(result) = ((self.get_position_13("*")) as usize).checked_sub(1)
                        {
                            result
                        } else {
                            0
                        },
                    )),
                    &*(self.get_item_13(
                        if let Some(result) =
                            ((self.get_position_13("*") + 1.0) as usize).checked_sub(1)
                        {
                            result
                        } else {
                            0
                        },
                    )),
                );
                *self.vf_1_10.lock().unwrap() = self.get_position_13("*") - 1.0;
                *self.vs_1_10.lock().unwrap() = (self.get_position_13("*") - 1.0).to_string();
                let repeatto = 2_usize;
                for _ in 0..repeatto {
                    let removein = (self.get_1_num_10() as i32 - 1) as usize;
                    if removein != usize::MAX {
                        if removein < self.l_1_13.lock().unwrap().len() {
                            self.l_1_13.lock().unwrap().remove(removein);
                        }
                    }
                }
                let getin = (self.get_1_num_10() as i32 - 1) as usize;
                let itemin = &*self.get_1_str_8();
                if getin != usize::MAX {
                    if let Some(item) = self.l_1_13.lock().unwrap().get_mut(getin) {
                        *item = itemin.to_owned();
                    };
                };
            } else {
                self.proc1(
                    &*(self.get_item_13(
                        if let Some(result) =
                            ((self.get_position_13("/") - 1.0) as usize).checked_sub(1)
                        {
                            result
                        } else {
                            0
                        },
                    )),
                    &*(self.get_item_13(
                        if let Some(result) = ((self.get_position_13("/")) as usize).checked_sub(1)
                        {
                            result
                        } else {
                            0
                        },
                    )),
                    &*(self.get_item_13(
                        if let Some(result) =
                            ((self.get_position_13("/") + 1.0) as usize).checked_sub(1)
                        {
                            result
                        } else {
                            0
                        },
                    )),
                );
                *self.vf_1_10.lock().unwrap() = self.get_position_13("/") - 1.0;
                *self.vs_1_10.lock().unwrap() = (self.get_position_13("/") - 1.0).to_string();
                let repeatto = 2_usize;
                for _ in 0..repeatto {
                    let removein = (self.get_1_num_10() as i32 - 1) as usize;
                    if removein != usize::MAX {
                        if removein < self.l_1_13.lock().unwrap().len() {
                            self.l_1_13.lock().unwrap().remove(removein);
                        }
                    }
                }
                let getin = (self.get_1_num_10() as i32 - 1) as usize;
                let itemin = &*self.get_1_str_8();
                if getin != usize::MAX {
                    if let Some(item) = self.l_1_13.lock().unwrap().get_mut(getin) {
                        *item = itemin.to_owned();
                    };
                };
            };
        }
    }

    fn proc3(&self, pm_17: &str) {
        while !(!(self.get_contains_13(String::from(pm_17)))) {
            self.proc1(
                &*(self.get_item_13(
                    if let Some(result) =
                        ((self.get_position_13(pm_17) - 1.0) as usize).checked_sub(1)
                    {
                        result
                    } else {
                        0
                    },
                )),
                &*(self.get_item_13(
                    if let Some(result) = ((self.get_position_13(pm_17)) as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                )),
                &*(self.get_item_13(
                    if let Some(result) =
                        ((self.get_position_13(pm_17) + 1.0) as usize).checked_sub(1)
                    {
                        result
                    } else {
                        0
                    },
                )),
            );
            *self.vf_1_10.lock().unwrap() = self.get_position_13(pm_17) - 1.0;
            *self.vs_1_10.lock().unwrap() = (self.get_position_13(pm_17) - 1.0).to_string();
            let repeatto = 2_usize;
            for _ in 0..repeatto {
                let removein = (self.get_1_num_10() as i32 - 1) as usize;
                if removein != usize::MAX {
                    if removein < self.l_1_13.lock().unwrap().len() {
                        self.l_1_13.lock().unwrap().remove(removein);
                    }
                }
            }
            let getin = (self.get_1_num_10() as i32 - 1) as usize;
            let itemin = &*self.get_1_str_8();
            if getin != usize::MAX {
                if let Some(item) = self.l_1_13.lock().unwrap().get_mut(getin) {
                    *item = itemin.to_owned();
                };
            };
        }
    }

    fn get_leng_13(&self) -> f64 {
        self.l_1_13.lock().unwrap().len() as f64
    }
    fn proc4(&self) {
        *self.vf_1_8.lock().unwrap() = get_f64_string("false");
        *self.vs_1_8.lock().unwrap() = String::from("false");
        if (self.get_leng_13()).to_string() == (1.0).to_string() {
            *self.vf_1_8.lock().unwrap() = get_f64_string(
                self.get_item_13(if let Some(result) = (1_usize).checked_sub(1) {
                    result
                } else {
                    0
                })
                .clone(),
            );
            *self.vs_1_8.lock().unwrap() =
                self.get_item_13(if let Some(result) = (1_usize).checked_sub(1) {
                    result
                } else {
                    0
                });
        } else {
            self.proc3("..");
            self.proc3(">=");
            self.proc3("<=");
            self.proc3("==");
            self.proc3("!=");
            self.proc3("||");
            self.proc3("&&");
            self.proc3("^");
            self.proc3("%");
            self.proc2();
            self.proc3("+");
        };
    }

    fn get_1_str_7(&self) -> String {
        let globalvar = self.vs_1_7.lock().unwrap().clone();
        globalvar
    }
    fn get_leng_1(&self) -> f64 {
        self.l_1.lock().unwrap().len() as f64
    }
    fn get_1_num_4(&self) -> f64 {
        let globalvar = *self.vf_1_4.lock().unwrap();
        globalvar
    }
    fn get_item_1(&self, item: usize) -> String {
        if let Some(value) = self.l_1.lock().unwrap().get(item) {
            value.to_string()
        } else {
            String::new()
        }
    }
    fn get_1_num_5(&self) -> f64 {
        let globalvar = *self.vf_1_5.lock().unwrap();
        globalvar
    }
    fn get_1_str_6(&self) -> String {
        let globalvar = self.vs_1_6.lock().unwrap().clone();
        globalvar
    }
    fn get_1_num_7(&self) -> f64 {
        let globalvar = *self.vf_1_7.lock().unwrap();
        globalvar
    }
    fn get_1_str_9(&self) -> String {
        let globalvar = self.vs_1_9.lock().unwrap().clone();
        globalvar
    }
    fn get_1_num_6(&self) -> f64 {
        let globalvar = *self.vf_1_6.lock().unwrap();
        globalvar
    }
    fn get_item_11(&self, item: usize) -> String {
        if let Some(value) = self.l_1_11.lock().unwrap().get(item) {
            value.to_string()
        } else {
            String::new()
        }
    }
    fn get_contains_1(&self, item: String) -> bool {
        self.l_1.lock().unwrap().contains(&item)
    }
    fn get_leng_11(&self) -> f64 {
        self.l_1_11.lock().unwrap().len() as f64
    }
    fn get_leng_12(&self) -> f64 {
        self.l_1_12.lock().unwrap().len() as f64
    }
    fn get_item_12(&self, item: usize) -> String {
        if let Some(value) = self.l_1_12.lock().unwrap().get(item) {
            value.to_string()
        } else {
            String::new()
        }
    }
    fn get_contains_12(&self, item: String) -> bool {
        self.l_1_12.lock().unwrap().contains(&item)
    }
    fn get_position_12(&self, value_to_find: &str) -> f64 {
        if let Some(position) = self
            .l_1_12
            .lock()
            .unwrap()
            .iter()
            .position(|x| x == &value_to_find)
        {
            position as f64 + 1.0
        } else {
            0.0
        }
    }
    fn get_1_num_9(&self) -> f64 {
        let globalvar = *self.vf_1_9.lock().unwrap();
        globalvar
    }
    fn proc5(&self, pm_18: &str) -> Arc<String> {
        *self.vf_1_7.lock().unwrap() = get_f64_string(pm_18);
        *self.vs_1_7.lock().unwrap() = String::from(pm_18);
        *self.vf_1_6.lock().unwrap() = 0.0;
        *self.vs_1_6.lock().unwrap() = (0.0).to_string();
        let foreachvarnum2 = *self.vf_1_4.lock().unwrap();
        let foreachvarstr2 = (*self.vs_1_4.lock().unwrap()).clone();
        *self.vs_1_4.lock().unwrap() = String::from("0");
        *self.vf_1_4.lock().unwrap() = 0.0;
        let foreachto2 = (self.get_1_str_7().chars().count() as f64) as usize;
        let mut foreachnum2 = 1;
        while foreachnum2 <= foreachto2 {
            *self.vf_1_4.lock().unwrap() = foreachnum2 as f64;
            *self.vs_1_4.lock().unwrap() = foreachnum2.to_string();
            let foreachvarnum1 = *self.vf_1_5.lock().unwrap();
            let foreachvarstr1 = (*self.vs_1_5.lock().unwrap()).clone();
            *self.vs_1_5.lock().unwrap() = String::from("0");
            *self.vf_1_5.lock().unwrap() = 0.0;
            let foreachto1 = (self.get_leng_1()) as usize;
            let mut foreachnum1 = 1;
            while foreachnum1 <= foreachto1 {
                *self.vf_1_5.lock().unwrap() = foreachnum1 as f64;
                *self.vs_1_5.lock().unwrap() = foreachnum1.to_string();
                if get_char_at(self.get_1_str_7(), self.get_1_num_4())
                    == self.get_item_1(
                        if let Some(result) = (self.get_1_num_5() as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                {
                    *self.vf_1_6.lock().unwrap() += 1.0;
                    *self.vs_1_6.lock().unwrap() = (*self.vf_1_6.lock().unwrap()).to_string();
                };
                foreachnum1 += 1;
            }
            *self.vf_1_5.lock().unwrap() = foreachvarnum1;
            *self.vs_1_5.lock().unwrap() = foreachvarstr1;
            foreachnum2 += 1;
        }
        *self.vf_1_4.lock().unwrap() = foreachvarnum2;
        *self.vs_1_4.lock().unwrap() = foreachvarstr2;
        if self.get_1_str_6() == (0.0).to_string() {
            *self.vf_1_8.lock().unwrap() = self.get_1_num_7();
            *self.vs_1_8.lock().unwrap() = self.get_1_str_7();
        } else {
            if !(get_char_at(self.get_1_str_7(), 1.0) == String::from("-"))
                && !(get_char_at(self.get_1_str_7(), 1.0) == String::from("!"))
            {
                *self.vf_1_7.lock().unwrap() =
                    get_f64_string("!".to_owned() + &self.get_1_str_7().to_owned().clone());
                *self.vs_1_7.lock().unwrap() = "!".to_owned() + &self.get_1_str_7().to_owned();
            };
            *self.vf_1_4.lock().unwrap() = 0.0;
            *self.vs_1_4.lock().unwrap() = (0.0).to_string();
            *self.vf_1_6.lock().unwrap() = 1.0;
            *self.vs_1_6.lock().unwrap() = (1.0).to_string();
            *self.vf_1_9.lock().unwrap() = 0.0;
            *self.vs_1_9.lock().unwrap() = (0.0).to_string();
            self.l_1_11.lock().unwrap().clear();
            let repeatto = (self.get_1_str_7().chars().count() as f64) as usize;
            for _ in 0..repeatto {
                *self.vf_1_4.lock().unwrap() += 1.0;
                *self.vs_1_4.lock().unwrap() = (*self.vf_1_4.lock().unwrap()).to_string();
                if (get_char_at(self.get_1_str_7(), self.get_1_num_4()) == String::from("\""))
                    || (get_char_at(self.get_1_str_7(), self.get_1_num_4()) == String::from("'"))
                {
                    if self.get_1_str_9() == (0.0).to_string() {
                        *self.vf_1_9.lock().unwrap() += 1.0;
                        *self.vs_1_9.lock().unwrap() = (*self.vf_1_9.lock().unwrap()).to_string();
                    } else {
                        *self.vf_1_9.lock().unwrap() += -1.0;
                        *self.vs_1_9.lock().unwrap() = (*self.vf_1_9.lock().unwrap()).to_string();
                    };
                };
                if self.get_1_str_9() == (0.0).to_string() {
                    let foreachvarnum3 = *self.vf_1_5.lock().unwrap();
                    let foreachvarstr3 = (*self.vs_1_5.lock().unwrap()).clone();
                    *self.vs_1_5.lock().unwrap() = String::from("0");
                    *self.vf_1_5.lock().unwrap() = 0.0;
                    let foreachto3 = (self.get_leng_1()) as usize;
                    let mut foreachnum3 = 1;
                    while foreachnum3 <= foreachto3 {
                        *self.vf_1_5.lock().unwrap() = foreachnum3 as f64;
                        *self.vs_1_5.lock().unwrap() = foreachnum3.to_string();
                        if (get_char_at(self.get_1_str_7(), self.get_1_num_4()).to_owned()
                            + &get_char_at(self.get_1_str_7(), self.get_1_num_4() + 1.0).to_owned())
                            == self.get_item_1(
                                if let Some(result) = (self.get_1_num_5() as usize).checked_sub(1) {
                                    result
                                } else {
                                    0
                                },
                            )
                        {
                            let addin = get_char_at(self.get_1_str_7(), self.get_1_num_4())
                                .to_owned()
                                + &get_char_at(self.get_1_str_7(), self.get_1_num_4() + 1.0)
                                    .to_owned();
                            self.l_1_11.lock().unwrap().push(addin);
                            *self.vf_1_6.lock().unwrap() += 1.0;
                            *self.vs_1_6.lock().unwrap() =
                                (*self.vf_1_6.lock().unwrap()).to_string();
                            *self.vf_1_4.lock().unwrap() += 1.0;
                            *self.vs_1_4.lock().unwrap() =
                                (*self.vf_1_4.lock().unwrap()).to_string();
                            if self.get_contains_1(
                                get_char_at(
                                    self.get_item_11(
                                        if let Some(result) =
                                            ((self.get_1_num_6() - 1.0) as usize).checked_sub(1)
                                        {
                                            result
                                        } else {
                                            0
                                        },
                                    ),
                                    1.0,
                                )
                                .to_owned()
                                    + &get_char_at(
                                        self.get_item_11(
                                            if let Some(result) =
                                                ((self.get_1_num_6() - 1.0) as usize).checked_sub(1)
                                            {
                                                result
                                            } else {
                                                0
                                            },
                                        ),
                                        2.0,
                                    )
                                    .to_owned(),
                            ) || (self.get_1_str_6() == (2.0).to_string())
                            {
                                *self.vf_1_6.lock().unwrap() += 1.0;
                                *self.vs_1_6.lock().unwrap() =
                                    (*self.vf_1_6.lock().unwrap()).to_string();
                            };
                            let getin = ((self.get_leng_11() - 1.0) as i32 - 1) as usize;
                            let itemin = &*(if 2_usize
                                != (self
                                    .get_item_11(
                                        if let Some(result) =
                                            ((self.get_leng_11() - 1.0) as usize).checked_sub(1)
                                        {
                                            result
                                        } else {
                                            0
                                        },
                                    )
                                    .chars()
                                    .count() as f64) as usize
                            {
                                if let Some(substring) = self
                                    .get_item_11(
                                        if let Some(result) =
                                            ((self.get_leng_11() - 1.0) as usize).checked_sub(1)
                                        {
                                            result
                                        } else {
                                            0
                                        },
                                    )
                                    .get(
                                        (2_i32 - 1) as usize
                                            ..=((self
                                                .get_item_11(
                                                    if let Some(result) =
                                                        ((self.get_leng_11() - 1.0) as usize)
                                                            .checked_sub(1)
                                                    {
                                                        result
                                                    } else {
                                                        0
                                                    },
                                                )
                                                .chars()
                                                .count()
                                                as f64)
                                                as i32
                                                - 1)
                                                as usize,
                                    )
                                {
                                    substring.to_string()
                                } else {
                                    String::new()
                                }
                            } else {
                                get_char_at(
                                    self.get_item_11(
                                        if let Some(result) =
                                            ((self.get_leng_11() - 1.0) as usize).checked_sub(1)
                                        {
                                            result
                                        } else {
                                            0
                                        },
                                    ),
                                    2.0,
                                )
                            });
                            if getin != usize::MAX {
                                if let Some(item) = self.l_1_11.lock().unwrap().get_mut(getin) {
                                    *item = itemin.to_owned();
                                };
                            };
                        } else {
                            if get_char_at(self.get_1_str_7(), self.get_1_num_4())
                                == self.get_item_1(
                                    if let Some(result) =
                                        (self.get_1_num_5() as usize).checked_sub(1)
                                    {
                                        result
                                    } else {
                                        0
                                    },
                                )
                            {
                                let addin = get_char_at(self.get_1_str_7(), self.get_1_num_4());
                                self.l_1_11.lock().unwrap().push(addin);
                                *self.vf_1_6.lock().unwrap() += 1.0;
                                *self.vs_1_6.lock().unwrap() =
                                    (*self.vf_1_6.lock().unwrap()).to_string();
                                if self.get_contains_1(get_char_at(
                                    self.get_item_11(
                                        if let Some(result) =
                                            ((self.get_1_num_6() - 1.0) as usize).checked_sub(1)
                                        {
                                            result
                                        } else {
                                            0
                                        },
                                    ),
                                    1.0,
                                )) || (self.get_1_str_6() == (2.0).to_string())
                                {
                                    *self.vf_1_6.lock().unwrap() += 1.0;
                                    *self.vs_1_6.lock().unwrap() =
                                        (*self.vf_1_6.lock().unwrap()).to_string();
                                };
                                let getin = ((self.get_leng_11() - 1.0) as i32 - 1) as usize;
                                let itemin = &*(if 2_usize
                                    != (self
                                        .get_item_11(
                                            if let Some(result) =
                                                ((self.get_leng_11() - 1.0) as usize).checked_sub(1)
                                            {
                                                result
                                            } else {
                                                0
                                            },
                                        )
                                        .chars()
                                        .count() as f64)
                                        as usize
                                {
                                    if let Some(substring) = self
                                        .get_item_11(
                                            if let Some(result) =
                                                ((self.get_leng_11() - 1.0) as usize).checked_sub(1)
                                            {
                                                result
                                            } else {
                                                0
                                            },
                                        )
                                        .get(
                                            (2_i32 - 1) as usize
                                                ..=((self
                                                    .get_item_11(
                                                        if let Some(result) =
                                                            ((self.get_leng_11() - 1.0) as usize)
                                                                .checked_sub(1)
                                                        {
                                                            result
                                                        } else {
                                                            0
                                                        },
                                                    )
                                                    .chars()
                                                    .count()
                                                    as f64)
                                                    as i32
                                                    - 1)
                                                    as usize,
                                        )
                                    {
                                        substring.to_string()
                                    } else {
                                        String::new()
                                    }
                                } else {
                                    get_char_at(
                                        self.get_item_11(
                                            if let Some(result) =
                                                ((self.get_leng_11() - 1.0) as usize).checked_sub(1)
                                            {
                                                result
                                            } else {
                                                0
                                            },
                                        ),
                                        2.0,
                                    )
                                });
                                if getin != usize::MAX {
                                    if let Some(item) = self.l_1_11.lock().unwrap().get_mut(getin) {
                                        *item = itemin.to_owned();
                                    };
                                };
                            };
                        };
                        foreachnum3 += 1;
                    }
                    *self.vf_1_5.lock().unwrap() = foreachvarnum3;
                    *self.vs_1_5.lock().unwrap() = foreachvarstr3;
                };
                let insertin = self
                    .get_item_11(
                        if let Some(result) = (self.get_1_num_6() as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                    .to_owned()
                    + &get_char_at(self.get_1_str_7(), self.get_1_num_4()).to_owned();
                let insertin2 = (self.get_1_num_6() as i32 - 1) as usize;
                if insertin2 != usize::MAX {
                    self.l_1_11.lock().unwrap().insert(insertin2, insertin);
                }
                let removein = ((self.get_1_num_6() + 1.0) as i32 - 1) as usize;
                if removein != usize::MAX {
                    if removein < self.l_1_11.lock().unwrap().len() {
                        self.l_1_11.lock().unwrap().remove(removein);
                    }
                }
            }
            let getin = ((self.get_leng_11()) as i32 - 1) as usize;
            let itemin = &*(if 2_usize
                != (self
                    .get_item_11(
                        if let Some(result) = ((self.get_leng_11()) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                    .chars()
                    .count() as f64) as usize
            {
                if let Some(substring) = self
                    .get_item_11(
                        if let Some(result) = ((self.get_leng_11()) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                    .get(
                        (2_i32 - 1) as usize
                            ..=((self
                                .get_item_11(
                                    if let Some(result) =
                                        ((self.get_leng_11()) as usize).checked_sub(1)
                                    {
                                        result
                                    } else {
                                        0
                                    },
                                )
                                .chars()
                                .count() as f64) as i32
                                - 1) as usize,
                    )
                {
                    substring.to_string()
                } else {
                    String::new()
                }
            } else {
                get_char_at(
                    self.get_item_11(
                        if let Some(result) = ((self.get_leng_11()) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    ),
                    2.0,
                )
            });
            if getin != usize::MAX {
                if let Some(item) = self.l_1_11.lock().unwrap().get_mut(getin) {
                    *item = itemin.to_owned();
                };
            };
            let foreachvarnum4 = *self.vf_1_4.lock().unwrap();
            let foreachvarstr4 = (*self.vs_1_4.lock().unwrap()).clone();
            *self.vs_1_4.lock().unwrap() = String::from("0");
            *self.vf_1_4.lock().unwrap() = 0.0;
            let foreachto4 = (self.get_leng_11()) as usize;
            let mut foreachnum4 = 1;
            while foreachnum4 <= foreachto4 {
                *self.vf_1_4.lock().unwrap() = foreachnum4 as f64;
                *self.vs_1_4.lock().unwrap() = foreachnum4.to_string();
                let getin = (self.get_1_num_4() as i32 - 1) as usize;
                let itemin = &*(self
                    .get_item_11(
                        if let Some(result) = (self.get_1_num_4() as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                    .replace(" ", ""));
                if getin != usize::MAX {
                    if let Some(item) = self.l_1_11.lock().unwrap().get_mut(getin) {
                        *item = itemin.to_owned();
                    };
                };
                if self.get_item_11(
                    if let Some(result) = (self.get_1_num_4() as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                ) == String::from("")
                {
                    let removein = (self.get_1_num_4() as i32 - 1) as usize;
                    if removein != usize::MAX {
                        if removein < self.l_1_11.lock().unwrap().len() {
                            self.l_1_11.lock().unwrap().remove(removein);
                        }
                    }
                };
                foreachnum4 += 1;
            }
            *self.vf_1_4.lock().unwrap() = foreachvarnum4;
            *self.vs_1_4.lock().unwrap() = foreachvarstr4;
            *self.vf_1_4.lock().unwrap() = 0.0;
            *self.vs_1_4.lock().unwrap() = (0.0).to_string();
            let repeatto = (self.get_leng_11()) as usize;
            for _ in 0..repeatto {
                *self.vf_1_4.lock().unwrap() += 1.0;
                *self.vs_1_4.lock().unwrap() = (*self.vf_1_4.lock().unwrap()).to_string();
                if self.get_item_11(
                    if let Some(result) = (self.get_1_num_4() as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                ) == String::from("-")
                {
                    if self.get_contains_1(self.get_item_11(
                        if let Some(result) = ((self.get_1_num_4() - 1.0) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )) {
                        let insertin = (0.0).to_string();
                        let insertin2 = (self.get_1_num_4() as i32 - 1) as usize;
                        if insertin2 != usize::MAX {
                            self.l_1_11.lock().unwrap().insert(insertin2, insertin);
                        }
                        *self.vf_1_4.lock().unwrap() += 1.0;
                        *self.vs_1_4.lock().unwrap() = (*self.vf_1_4.lock().unwrap()).to_string();
                    };
                    let getin = (self.get_1_num_4() as i32 - 1) as usize;
                    let itemin = "+";
                    if getin != usize::MAX {
                        if let Some(item) = self.l_1_11.lock().unwrap().get_mut(getin) {
                            *item = itemin.to_owned();
                        };
                    };
                    let getin = ((self.get_1_num_4() + 1.0) as i32 - 1) as usize;
                    let itemin = &*("-".to_owned()
                        + &self
                            .get_item_11(
                                if let Some(result) =
                                    ((self.get_1_num_4() + 1.0) as usize).checked_sub(1)
                                {
                                    result
                                } else {
                                    0
                                },
                            )
                            .to_owned());
                    if getin != usize::MAX {
                        if let Some(item) = self.l_1_11.lock().unwrap().get_mut(getin) {
                            *item = itemin.to_owned();
                        };
                    };
                    *self.vf_1_4.lock().unwrap() += 1.0;
                    *self.vs_1_4.lock().unwrap() = (*self.vf_1_4.lock().unwrap()).to_string();
                };
            }
            *self.vf_1_6.lock().unwrap() = 0.0;
            *self.vs_1_6.lock().unwrap() = (0.0).to_string();
            *self.vf_1_4.lock().unwrap() = 0.0;
            *self.vs_1_4.lock().unwrap() = (0.0).to_string();
            *self.vf_1_9.lock().unwrap() = 0.0;
            *self.vs_1_9.lock().unwrap() = (0.0).to_string();
            let repeatto = (self.get_leng_11()) as usize;
            for _ in 0..repeatto {
                *self.vf_1_4.lock().unwrap() += 1.0;
                *self.vs_1_4.lock().unwrap() = (*self.vf_1_4.lock().unwrap()).to_string();
                if self.get_item_11(
                    if let Some(result) = (self.get_1_num_4() as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                ) == String::from("(")
                {
                    *self.vf_1_6.lock().unwrap() += 1.0;
                    *self.vs_1_6.lock().unwrap() = (*self.vf_1_6.lock().unwrap()).to_string();
                    let removein = (self.get_1_num_4() as i32 - 1) as usize;
                    if removein != usize::MAX {
                        if removein < self.l_1_11.lock().unwrap().len() {
                            self.l_1_11.lock().unwrap().remove(removein);
                        }
                    }
                    *self.vf_1_4.lock().unwrap() += -1.0;
                    *self.vs_1_4.lock().unwrap() = (*self.vf_1_4.lock().unwrap()).to_string();
                } else {
                    if self.get_item_11(
                        if let Some(result) = (self.get_1_num_4() as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    ) == String::from(")")
                    {
                        *self.vf_1_6.lock().unwrap() += -1.0;
                        *self.vs_1_6.lock().unwrap() = (*self.vf_1_6.lock().unwrap()).to_string();
                        let removein = (self.get_1_num_4() as i32 - 1) as usize;
                        if removein != usize::MAX {
                            if removein < self.l_1_11.lock().unwrap().len() {
                                self.l_1_11.lock().unwrap().remove(removein);
                            }
                        }
                    } else {
                        let getin = (self.get_1_num_4() as i32 - 1) as usize;
                        let itemin = &*(repeat(" ", self.get_1_num_6() as usize).to_owned()
                            + &self
                                .get_item_11(
                                    if let Some(result) =
                                        (self.get_1_num_4() as usize).checked_sub(1)
                                    {
                                        result
                                    } else {
                                        0
                                    },
                                )
                                .to_owned());
                        if getin != usize::MAX {
                            if let Some(item) = self.l_1_11.lock().unwrap().get_mut(getin) {
                                *item = itemin.to_owned();
                            };
                        };
                    };
                };
            }
            self.l_1_12.lock().unwrap().clear();
            *self.vf_1_6.lock().unwrap() = 0.0;
            *self.vs_1_6.lock().unwrap() = (0.0).to_string();
            let foreachvarnum5 = *self.vf_1_4.lock().unwrap();
            let foreachvarstr5 = (*self.vs_1_4.lock().unwrap()).clone();
            *self.vs_1_4.lock().unwrap() = String::from("0");
            *self.vf_1_4.lock().unwrap() = 0.0;
            let foreachto5 = (self.get_leng_11()) as usize;
            let mut foreachnum5 = 1;
            while foreachnum5 <= foreachto5 {
                *self.vf_1_4.lock().unwrap() = foreachnum5 as f64;
                *self.vs_1_4.lock().unwrap() = foreachnum5.to_string();
                if (self
                    .get_item_11(
                        if let Some(result) = (self.get_1_num_4() as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                    .matches(" ")
                    .count() as f64)
                    > self.get_1_num_6()
                {
                    *self.vf_1_6.lock().unwrap() += 1.0;
                    *self.vs_1_6.lock().unwrap() = (*self.vf_1_6.lock().unwrap()).to_string();
                    let addin = self.get_1_str_6();
                    self.l_1_12.lock().unwrap().push(addin);
                } else {
                    if (self
                        .get_item_11(
                            if let Some(result) = (self.get_1_num_4() as usize).checked_sub(1) {
                                result
                            } else {
                                0
                            },
                        )
                        .matches(" ")
                        .count() as f64)
                        < self.get_1_num_6()
                    {
                        *self.vf_1_6.lock().unwrap() += -1.0;
                        *self.vs_1_6.lock().unwrap() = (*self.vf_1_6.lock().unwrap()).to_string();
                        let addin = self.get_1_str_6();
                        self.l_1_12.lock().unwrap().push(addin);
                    } else {
                        let addin = String::from("");
                        self.l_1_12.lock().unwrap().push(addin);
                    };
                };
                foreachnum5 += 1;
            }
            *self.vf_1_4.lock().unwrap() = foreachvarnum5;
            *self.vs_1_4.lock().unwrap() = foreachvarstr5;
            let addin = (0.0).to_string();
            self.l_1_12.lock().unwrap().push(addin);
            *self.vf_1_6.lock().unwrap() = 0.0;
            *self.vs_1_6.lock().unwrap() = (0.0).to_string();
            let foreachvarnum6 = *self.vf_1_4.lock().unwrap();
            let foreachvarstr6 = (*self.vs_1_4.lock().unwrap()).clone();
            *self.vs_1_4.lock().unwrap() = String::from("0");
            *self.vf_1_4.lock().unwrap() = 0.0;
            let foreachto6 = (self.get_leng_12()) as usize;
            let mut foreachnum6 = 1;
            while foreachnum6 <= foreachto6 {
                *self.vf_1_4.lock().unwrap() = foreachnum6 as f64;
                *self.vs_1_4.lock().unwrap() = foreachnum6.to_string();
                if get_f64_string(
                    self.get_item_12(
                        if let Some(result) = (self.get_1_num_4() as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                    .clone(),
                ) > self.get_1_num_6()
                {
                    *self.vf_1_6.lock().unwrap() += 1.0;
                    *self.vs_1_6.lock().unwrap() = (*self.vf_1_6.lock().unwrap()).to_string();
                };
                foreachnum6 += 1;
            }
            *self.vf_1_4.lock().unwrap() = foreachvarnum6;
            *self.vs_1_4.lock().unwrap() = foreachvarstr6;
            *self.vf_1_9.lock().unwrap() = self.get_1_num_6();
            *self.vs_1_9.lock().unwrap() = self.get_1_str_6();
            let repeatto = self.get_1_num_6() as usize;
            for _ in 0..repeatto {
                while !(!(self.get_contains_12(self.get_1_str_9()))) {
                    self.l_1_13.lock().unwrap().clear();
                    *self.vf_1_4.lock().unwrap() = self.get_position_12(&*self.get_1_str_9());
                    *self.vs_1_4.lock().unwrap() =
                        (self.get_position_12(&*self.get_1_str_9())).to_string();
                    let repeatto = (self.get_position_12(&((self.get_1_num_9() - 1.0).to_string()))
                        - self.get_position_12(&*self.get_1_str_9()))
                        as usize;
                    for _ in 0..repeatto {
                        let addin = self.get_item_11(
                            if let Some(result) = (self.get_1_num_4() as usize).checked_sub(1) {
                                result
                            } else {
                                0
                            },
                        );
                        self.l_1_13.lock().unwrap().push(addin);
                        let removein = (self.get_1_num_4() as i32 - 1) as usize;
                        if removein != usize::MAX {
                            if removein < self.l_1_11.lock().unwrap().len() {
                                self.l_1_11.lock().unwrap().remove(removein);
                            }
                        }
                        let removein = (self.get_1_num_4() as i32 - 1) as usize;
                        if removein != usize::MAX {
                            if removein < self.l_1_12.lock().unwrap().len() {
                                self.l_1_12.lock().unwrap().remove(removein);
                            }
                        }
                    }
                    let foreachvarnum7 = *self.vf_1_5.lock().unwrap();
                    let foreachvarstr7 = (*self.vs_1_5.lock().unwrap()).clone();
                    *self.vs_1_5.lock().unwrap() = String::from("0");
                    *self.vf_1_5.lock().unwrap() = 0.0;
                    let foreachto7 = (self.get_leng_13()) as usize;
                    let mut foreachnum7 = 1;
                    while foreachnum7 <= foreachto7 {
                        *self.vf_1_5.lock().unwrap() = foreachnum7 as f64;
                        *self.vs_1_5.lock().unwrap() = foreachnum7.to_string();
                        let getin = (self.get_1_num_5() as i32 - 1) as usize;
                        let itemin = &*(self
                            .get_item_13(
                                if let Some(result) = (self.get_1_num_5() as usize).checked_sub(1) {
                                    result
                                } else {
                                    0
                                },
                            )
                            .replace(" ", ""));
                        if getin != usize::MAX {
                            if let Some(item) = self.l_1_13.lock().unwrap().get_mut(getin) {
                                *item = itemin.to_owned();
                            };
                        };
                        if self.get_item_13(
                            if let Some(result) = (self.get_1_num_5() as usize).checked_sub(1) {
                                result
                            } else {
                                0
                            },
                        ) == String::from("")
                        {
                            let removein = (self.get_1_num_5() as i32 - 1) as usize;
                            if removein != usize::MAX {
                                if removein < self.l_1_13.lock().unwrap().len() {
                                    self.l_1_13.lock().unwrap().remove(removein);
                                }
                            }
                        };
                        foreachnum7 += 1;
                    }
                    *self.vf_1_5.lock().unwrap() = foreachvarnum7;
                    *self.vs_1_5.lock().unwrap() = foreachvarstr7;
                    self.proc4();
                    let insertin = String::from("");
                    let insertin2 = (self.get_1_num_4() as i32 - 1) as usize;
                    if insertin2 != usize::MAX {
                        self.l_1_12.lock().unwrap().insert(insertin2, insertin);
                    }
                    let getin = ((self.get_position_12(&((self.get_1_num_6() - 1.0).to_string())))
                        as i32
                        - 1) as usize;
                    let itemin = "";
                    if getin != usize::MAX {
                        if let Some(item) = self.l_1_12.lock().unwrap().get_mut(getin) {
                            *item = itemin.to_owned();
                        };
                    };
                    let insertin = self.get_1_str_8();
                    let insertin2 = (self.get_1_num_4() as i32 - 1) as usize;
                    if insertin2 != usize::MAX {
                        self.l_1_11.lock().unwrap().insert(insertin2, insertin);
                    }
                }
                *self.vf_1_9.lock().unwrap() += -1.0;
                *self.vs_1_9.lock().unwrap() = (*self.vf_1_9.lock().unwrap()).to_string();
            }
            self.l_1_13.lock().unwrap().clear();
            let foreachvarnum8 = *self.vf_1_4.lock().unwrap();
            let foreachvarstr8 = (*self.vs_1_4.lock().unwrap()).clone();
            *self.vs_1_4.lock().unwrap() = String::from("0");
            *self.vf_1_4.lock().unwrap() = 0.0;
            let foreachto8 = (self.get_leng_11()) as usize;
            let mut foreachnum8 = 1;
            while foreachnum8 <= foreachto8 {
                *self.vf_1_4.lock().unwrap() = foreachnum8 as f64;
                *self.vs_1_4.lock().unwrap() = foreachnum8.to_string();
                let addin = self.get_item_11(
                    if let Some(result) = (self.get_1_num_4() as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                );
                self.l_1_13.lock().unwrap().push(addin);
                foreachnum8 += 1;
            }
            *self.vf_1_4.lock().unwrap() = foreachvarnum8;
            *self.vs_1_4.lock().unwrap() = foreachvarstr8;
            self.proc4();
            self.l_1_11.lock().unwrap().clear();
            return Arc::new(self.get_1_str_8());
        };
        Arc::new(self.get_1_str_8())
    }
}
pub fn calc<T: AsRef<str>>(expression: T) -> f64 {
    let init = Default::new();
    get_f64_string(Arc::as_ref(&init.proc5(expression.as_ref())))
}
