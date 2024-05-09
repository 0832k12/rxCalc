// By r-Scratch-Compiler 2024-05-09T09:54:01.609Z 🍥 v1-stl
use std::sync::Arc;
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
    l_1: Vec<String>,
    vf_1_2: f64,
    vs_1_2: String,
    vf_1_3: f64,
    vs_1_3: String,
    vf_1_4: f64,
    vs_1_4: String,
    vf_1_5: f64,
    vs_1_5: String,
    vf_1_6: f64,
    vs_1_6: String,
    vf_1_7: f64,
    vs_1_7: String,
    vf_1_8: f64,
    vs_1_8: String,
    l_1_9: Vec<String>,
    l_1_10: Vec<String>,
    l_1_11: Vec<String>
}
impl Default {
    fn new() -> Default {
        Default {
            l_1: [
                "==", ">=", "<=", "!=", ">", "<", "+", "-", "*", "/", "%", "^", "&&", "||", "..",
                "(", ")",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            vf_1_2: 3.0,
            vs_1_2: (3).to_string(),
            vf_1_3: 17.0,
            vs_1_3: (17).to_string(),
            vf_1_4: 1.0,
            vs_1_4: (1).to_string(),
            vf_1_5: 0.0,
            vs_1_5: ("1+1").to_string(),
            vf_1_6: 2.0,
            vs_1_6: (2).to_string(),
            vf_1_7: 0.0,
            vs_1_7: (0).to_string(),
            vf_1_8: 1.0,
            vs_1_8: (1).to_string(),
            l_1_9: vec![],
            l_1_10: ["", "", "", "0", "东西", "东西", "东西"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            l_1_11: ["2"].iter().map(|s| s.to_string()).collect()
        }
    }
    fn proc1(&mut self, pm_12: &str, pm_13: &str, pm_14: &str) {
        self.vf_1_6 = 0.0;
        self.vs_1_6 = (0.0).to_string();
        if pm_13.contains(&"==") {
            if String::from(pm_12) == String::from(pm_14) {
                self.vf_1_6 = 1.0;
                self.vs_1_6 = (1.0).to_string();
            } else {
                self.vf_1_6 = 0.0;
                self.vs_1_6 = (0.0).to_string();
            };
        } else {
            if pm_13.contains(&"!=") {
                if !(String::from(pm_12) == String::from(pm_14)) {
                    self.vf_1_6 = 1.0;
                    self.vs_1_6 = (1.0).to_string();
                } else {
                    self.vf_1_6 = 0.0;
                    self.vs_1_6 = (0.0).to_string();
                };
            } else {
                if pm_13.contains(&"<") {
                    if get_f64_string(pm_12) < get_f64_string(pm_14) {
                        self.vf_1_6 = 1.0;
                        self.vs_1_6 = (1.0).to_string();
                    } else {
                        self.vf_1_6 = 0.0;
                        self.vs_1_6 = (0.0).to_string();
                    };
                } else {
                    if pm_13.contains(&">") {
                        if get_f64_string(pm_12) > get_f64_string(pm_14) {
                            self.vf_1_6 = 1.0;
                            self.vs_1_6 = (1.0).to_string();
                        } else {
                            self.vf_1_6 = 0.0;
                            self.vs_1_6 = (0.0).to_string();
                        };
                    } else {
                        if pm_13.contains(&"<=") {
                            if get_f64_string(pm_12) <= get_f64_string(pm_14) {
                                self.vf_1_6 = 1.0;
                                self.vs_1_6 = (1.0).to_string();
                            } else {
                                self.vf_1_6 = 0.0;
                                self.vs_1_6 = (0.0).to_string();
                            };
                        } else {
                            if pm_13.contains(&">=") {
                                if get_f64_string(pm_12) >= get_f64_string(pm_14) {
                                    self.vf_1_6 = 1.0;
                                    self.vs_1_6 = (1.0).to_string();
                                } else {
                                    self.vf_1_6 = 0.0;
                                    self.vs_1_6 = (0.0).to_string();
                                };
                            } else {
                                if pm_13.contains(&"+") {
                                    self.vf_1_6 = get_f64_string(pm_12) + get_f64_string(pm_14);
                                    self.vs_1_6 =
                                        (get_f64_string(pm_12) + get_f64_string(pm_14)).to_string();
                                } else {
                                    if pm_13.contains(&"/") {
                                        self.vf_1_6 = get_f64_string(pm_12) / get_f64_string(pm_14);
                                        self.vs_1_6 = (get_f64_string(pm_12)
                                            / get_f64_string(pm_14))
                                        .to_string();
                                    } else {
                                        if pm_13.contains(&"*") {
                                            self.vf_1_6 =
                                                get_f64_string(pm_12) * get_f64_string(pm_14);
                                            self.vs_1_6 = (get_f64_string(pm_12)
                                                * get_f64_string(pm_14))
                                            .to_string();
                                        } else {
                                            if pm_13.contains(&"%") {
                                                self.vf_1_6 =
                                                    get_f64_string(pm_12) % get_f64_string(pm_14);
                                                self.vs_1_6 = (get_f64_string(pm_12)
                                                    % get_f64_string(pm_14))
                                                .to_string();
                                            } else {
                                                if pm_13.contains(&"||") {
                                                    self.vf_1_6 = get_f64_string(
                                                        if string_to_boolean(pm_12)
                                                            || string_to_boolean(pm_14)
                                                        {
                                                            (1.0).to_string()
                                                        } else {
                                                            (0.0).to_string()
                                                        }
                                                        .clone(),
                                                    );
                                                    self.vs_1_6 = if string_to_boolean(pm_12)
                                                        || string_to_boolean(pm_14)
                                                    {
                                                        (1.0).to_string()
                                                    } else {
                                                        (0.0).to_string()
                                                    };
                                                } else {
                                                    if pm_13.contains(&"&&") {
                                                        self.vf_1_6 = get_f64_string(
                                                            if string_to_boolean(pm_12)
                                                                && string_to_boolean(pm_14)
                                                            {
                                                                (1.0).to_string()
                                                            } else {
                                                                (0.0).to_string()
                                                            }
                                                            .clone(),
                                                        );
                                                        self.vs_1_6 = if string_to_boolean(pm_12)
                                                            && string_to_boolean(pm_14)
                                                        {
                                                            (1.0).to_string()
                                                        } else {
                                                            (0.0).to_string()
                                                        };
                                                    } else {
                                                        if pm_13.contains(&"..") {
                                                            self.vf_1_6 = get_f64_string(
                                                                pm_12.to_owned()
                                                                    + &pm_14.to_owned().clone(),
                                                            );
                                                            self.vs_1_6 = pm_12.to_owned()
                                                                + &pm_14.to_owned();
                                                        } else {
                                                            if pm_13.contains(&"^") {
                                                                self.vf_1_6 = get_f64_string(pm_12);
                                                                self.vs_1_6 = String::from(pm_12);
                                                                let repeatto =
                                                                    (get_f64_string(pm_12) - 1.0)
                                                                        as usize;
                                                                for _ in 0..repeatto {
                                                                    self.vf_1_6 = self.vf_1_6
                                                                        * get_f64_string(pm_12);
                                                                    self.vs_1_6 = (self.vf_1_6
                                                                        * get_f64_string(pm_12))
                                                                    .to_string();
                                                                }
                                                            } else {
                                                                if (String::from(pm_13)
                                                                    == String::from(""))
                                                                    && (String::from(pm_14)
                                                                        == String::from(""))
                                                                {
                                                                    self.vf_1_6 =
                                                                        get_f64_string(pm_12);
                                                                    self.vs_1_6 =
                                                                        String::from(pm_12);
                                                                } else {
                                                                    self.vf_1_6 =
                                                                        get_f64_string("Error");
                                                                    self.vs_1_6 =
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

    fn get_contains_11(&self, item: String) -> bool {
        self.l_1_11.contains(&item)
    }
    fn get_position_11(&self, value_to_find: &str) -> f64 {
        if let Some(position) = self.l_1_11.iter().position(|x| x == &value_to_find) {
            position as f64 + 1.0
        } else {
            0.0
        }
    }
    fn get_item_11(&self, item: usize) -> String {
        if let Some(value) = self.l_1_11.get(item) {
            value.to_string()
        } else {
            String::new()
        }
    }
    fn proc2(&mut self) {
        while !(!(self.get_contains_11(String::from("*")))
            && !(self.get_contains_11(String::from("/"))))
        {
            if self.get_position_11("*") > self.get_position_11("/") {
                self.proc1(
                    &*(self.get_item_11(
                        if let Some(result) =
                            ((self.get_position_11("*") - 1.0) as usize).checked_sub(1)
                        {
                            result
                        } else {
                            0
                        },
                    )),
                    &*(self.get_item_11(
                        if let Some(result) = ((self.get_position_11("*")) as usize).checked_sub(1)
                        {
                            result
                        } else {
                            0
                        },
                    )),
                    &*(self.get_item_11(
                        if let Some(result) =
                            ((self.get_position_11("*") + 1.0) as usize).checked_sub(1)
                        {
                            result
                        } else {
                            0
                        },
                    )),
                );
                self.vf_1_8 = self.get_position_11("*") - 1.0;
                self.vs_1_8 = (self.get_position_11("*") - 1.0).to_string();
                let repeatto = 2_usize;
                for _ in 0..repeatto {
                    let removein = (self.vf_1_8 as i32 - 1) as usize;
                    if removein != usize::MAX {
                        if removein < self.l_1_11.len() {
                            self.l_1_11.remove(removein);
                        }
                    }
                }
                let getin = (self.vf_1_8 as i32 - 1) as usize;
                let itemin = &*self.vs_1_6;
                if getin != usize::MAX {
                    if let Some(item) = self.l_1_11.get_mut(getin) {
                        *item = itemin.to_owned();
                    };
                };
            } else {
                self.proc1(
                    &*(self.get_item_11(
                        if let Some(result) =
                            ((self.get_position_11("/") - 1.0) as usize).checked_sub(1)
                        {
                            result
                        } else {
                            0
                        },
                    )),
                    &*(self.get_item_11(
                        if let Some(result) = ((self.get_position_11("/")) as usize).checked_sub(1)
                        {
                            result
                        } else {
                            0
                        },
                    )),
                    &*(self.get_item_11(
                        if let Some(result) =
                            ((self.get_position_11("/") + 1.0) as usize).checked_sub(1)
                        {
                            result
                        } else {
                            0
                        },
                    )),
                );
                self.vf_1_8 = self.get_position_11("/") - 1.0;
                self.vs_1_8 = (self.get_position_11("/") - 1.0).to_string();
                let repeatto = 2_usize;
                for _ in 0..repeatto {
                    let removein = (self.vf_1_8 as i32 - 1) as usize;
                    if removein != usize::MAX {
                        if removein < self.l_1_11.len() {
                            self.l_1_11.remove(removein);
                        }
                    }
                }
                let getin = (self.vf_1_8 as i32 - 1) as usize;
                let itemin = &*self.vs_1_6;
                if getin != usize::MAX {
                    if let Some(item) = self.l_1_11.get_mut(getin) {
                        *item = itemin.to_owned();
                    };
                };
            };
        }
    }

    fn proc3(&mut self, pm_15: &str) {
        while !(!(self.get_contains_11(String::from(pm_15)))) {
            self.proc1(
                &*(self.get_item_11(
                    if let Some(result) =
                        ((self.get_position_11(pm_15) - 1.0) as usize).checked_sub(1)
                    {
                        result
                    } else {
                        0
                    },
                )),
                &*(self.get_item_11(
                    if let Some(result) = ((self.get_position_11(pm_15)) as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                )),
                &*(self.get_item_11(
                    if let Some(result) =
                        ((self.get_position_11(pm_15) + 1.0) as usize).checked_sub(1)
                    {
                        result
                    } else {
                        0
                    },
                )),
            );
            self.vf_1_8 = self.get_position_11(pm_15) - 1.0;
            self.vs_1_8 = (self.get_position_11(pm_15) - 1.0).to_string();
            let repeatto = 2_usize;
            for _ in 0..repeatto {
                let removein = (self.vf_1_8 as i32 - 1) as usize;
                if removein != usize::MAX {
                    if removein < self.l_1_11.len() {
                        self.l_1_11.remove(removein);
                    }
                }
            }
            let getin = (self.vf_1_8 as i32 - 1) as usize;
            let itemin = &*self.vs_1_6;
            if getin != usize::MAX {
                if let Some(item) = self.l_1_11.get_mut(getin) {
                    *item = itemin.to_owned();
                };
            };
        }
    }

    fn get_leng_11(&self) -> f64 {
        self.l_1_11.len() as f64
    }
    fn proc4(&mut self) {
        self.vf_1_6 = get_f64_string("false");
        self.vs_1_6 = String::from("false");
        if (self.get_leng_11()).to_string() == (1.0).to_string() {
            self.vf_1_6 = get_f64_string(
                self.get_item_11(if let Some(result) = (1_usize).checked_sub(1) {
                    result
                } else {
                    0
                })
                .clone(),
            );
            self.vs_1_6 = self.get_item_11(if let Some(result) = (1_usize).checked_sub(1) {
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

    fn get_leng_1(&self) -> f64 {
        self.l_1.len() as f64
    }
    fn get_item_1(&self, item: usize) -> String {
        if let Some(value) = self.l_1.get(item) {
            value.to_string()
        } else {
            String::new()
        }
    }
    fn get_item_9(&self, item: usize) -> String {
        if let Some(value) = self.l_1_9.get(item) {
            value.to_string()
        } else {
            String::new()
        }
    }
    fn get_contains_1(&self, item: String) -> bool {
        self.l_1.contains(&item)
    }
    fn get_leng_9(&self) -> f64 {
        self.l_1_9.len() as f64
    }
    fn get_leng_10(&self) -> f64 {
        self.l_1_10.len() as f64
    }
    fn get_item_10(&self, item: usize) -> String {
        if let Some(value) = self.l_1_10.get(item) {
            value.to_string()
        } else {
            String::new()
        }
    }
    fn get_contains_10(&self, item: String) -> bool {
        self.l_1_10.contains(&item)
    }
    fn get_position_10(&self, value_to_find: &str) -> f64 {
        if let Some(position) = self.l_1_10.iter().position(|x| x == &value_to_find) {
            position as f64 + 1.0
        } else {
            0.0
        }
    }
    fn proc5(&mut self, pm_16: &str) -> Arc<String> {
        self.vf_1_5 = get_f64_string(pm_16);
        self.vs_1_5 = String::from(pm_16);
        self.vf_1_4 = 0.0;
        self.vs_1_4 = (0.0).to_string();
        let foreachvarnum2 = self.vf_1_2;
        let foreachvarstr2 = (self.vs_1_2).clone();
        self.vs_1_2 = String::from("0");
        self.vf_1_2 = 0.0;
        let foreachto2 = (self.vs_1_5.clone().chars().count() as f64) as usize;
        let mut foreachnum2 = 1;
        while foreachnum2 <= foreachto2 {
            self.vf_1_2 = foreachnum2 as f64;
            self.vs_1_2 = foreachnum2.to_string();
            let foreachvarnum1 = self.vf_1_3;
            let foreachvarstr1 = (self.vs_1_3).clone();
            self.vs_1_3 = String::from("0");
            self.vf_1_3 = 0.0;
            let foreachto1 = (self.get_leng_1()) as usize;
            let mut foreachnum1 = 1;
            while foreachnum1 <= foreachto1 {
                self.vf_1_3 = foreachnum1 as f64;
                self.vs_1_3 = foreachnum1.to_string();
                if get_char_at(self.vs_1_5.clone(), self.vf_1_2)
                    == self.get_item_1(
                        if let Some(result) = (self.vf_1_3 as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                {
                    self.vf_1_4 += 1.0;
                    self.vs_1_4 = (self.vf_1_4).to_string();
                };
                foreachnum1 += 1;
            }
            self.vf_1_3 = foreachvarnum1;
            self.vs_1_3 = foreachvarstr1;
            foreachnum2 += 1;
        }
        self.vf_1_2 = foreachvarnum2;
        self.vs_1_2 = foreachvarstr2;
        if self.vs_1_4.clone() == (0.0).to_string() {
            self.vf_1_6 = self.vf_1_5;
            self.vs_1_6 = self.vs_1_5.clone();
        } else {
            if !(get_char_at(self.vs_1_5.clone(), 1.0) == String::from("-"))
                && !(get_char_at(self.vs_1_5.clone(), 1.0) == String::from("!"))
            {
                self.vf_1_5 =
                    get_f64_string("!".to_owned() + &self.vs_1_5.clone().to_owned().clone());
                self.vs_1_5 = "!".to_owned() + &self.vs_1_5.clone().to_owned();
            };
            self.vf_1_2 = 0.0;
            self.vs_1_2 = (0.0).to_string();
            self.vf_1_4 = 1.0;
            self.vs_1_4 = (1.0).to_string();
            self.vf_1_7 = 0.0;
            self.vs_1_7 = (0.0).to_string();
            self.l_1_9.clear();
            let repeatto = (self.vs_1_5.clone().chars().count() as f64) as usize;
            for _ in 0..repeatto {
                self.vf_1_2 += 1.0;
                self.vs_1_2 = (self.vf_1_2).to_string();
                if (get_char_at(self.vs_1_5.clone(), self.vf_1_2) == String::from("\""))
                    || (get_char_at(self.vs_1_5.clone(), self.vf_1_2) == String::from("'"))
                {
                    if self.vs_1_7.clone() == (0.0).to_string() {
                        self.vf_1_7 += 1.0;
                        self.vs_1_7 = (self.vf_1_7).to_string();
                    } else {
                        self.vf_1_7 += -1.0;
                        self.vs_1_7 = (self.vf_1_7).to_string();
                    };
                };
                if self.vs_1_7.clone() == (0.0).to_string() {
                    let foreachvarnum3 = self.vf_1_3;
                    let foreachvarstr3 = (self.vs_1_3).clone();
                    self.vs_1_3 = String::from("0");
                    self.vf_1_3 = 0.0;
                    let foreachto3 = (self.get_leng_1()) as usize;
                    let mut foreachnum3 = 1;
                    while foreachnum3 <= foreachto3 {
                        self.vf_1_3 = foreachnum3 as f64;
                        self.vs_1_3 = foreachnum3.to_string();
                        if (get_char_at(self.vs_1_5.clone(), self.vf_1_2).to_owned()
                            + &get_char_at(self.vs_1_5.clone(), self.vf_1_2 + 1.0).to_owned())
                            == self.get_item_1(
                                if let Some(result) = (self.vf_1_3 as usize).checked_sub(1) {
                                    result
                                } else {
                                    0
                                },
                            )
                        {
                            let addin = get_char_at(self.vs_1_5.clone(), self.vf_1_2).to_owned()
                                + &get_char_at(self.vs_1_5.clone(), self.vf_1_2 + 1.0).to_owned();
                            self.l_1_9.push(addin);
                            self.vf_1_4 += 1.0;
                            self.vs_1_4 = (self.vf_1_4).to_string();
                            self.vf_1_2 += 1.0;
                            self.vs_1_2 = (self.vf_1_2).to_string();
                            if self.get_contains_1(
                                get_char_at(
                                    self.get_item_9(
                                        if let Some(result) =
                                            ((self.vf_1_4 - 1.0) as usize).checked_sub(1)
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
                                        self.get_item_9(
                                            if let Some(result) =
                                                ((self.vf_1_4 - 1.0) as usize).checked_sub(1)
                                            {
                                                result
                                            } else {
                                                0
                                            },
                                        ),
                                        2.0,
                                    )
                                    .to_owned(),
                            ) || (self.vs_1_4.clone() == (2.0).to_string())
                            {
                                self.vf_1_4 += 1.0;
                                self.vs_1_4 = (self.vf_1_4).to_string();
                            };
                            let getin = ((self.get_leng_9() - 1.0) as i32 - 1) as usize;
                            let itemin = &*(if 2_usize
                                != (self
                                    .get_item_9(
                                        if let Some(result) =
                                            ((self.get_leng_9() - 1.0) as usize).checked_sub(1)
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
                                    .get_item_9(
                                        if let Some(result) =
                                            ((self.get_leng_9() - 1.0) as usize).checked_sub(1)
                                        {
                                            result
                                        } else {
                                            0
                                        },
                                    )
                                    .get(
                                        (2_i32 - 1) as usize
                                            ..=((self
                                                .get_item_9(
                                                    if let Some(result) = ((self.get_leng_9() - 1.0)
                                                        as usize)
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
                                    self.get_item_9(
                                        if let Some(result) =
                                            ((self.get_leng_9() - 1.0) as usize).checked_sub(1)
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
                                if let Some(item) = self.l_1_9.get_mut(getin) {
                                    *item = itemin.to_owned();
                                };
                            };
                        } else {
                            if get_char_at(self.vs_1_5.clone(), self.vf_1_2)
                                == self.get_item_1(
                                    if let Some(result) = (self.vf_1_3 as usize).checked_sub(1) {
                                        result
                                    } else {
                                        0
                                    },
                                )
                            {
                                let addin = get_char_at(self.vs_1_5.clone(), self.vf_1_2);
                                self.l_1_9.push(addin);
                                self.vf_1_4 += 1.0;
                                self.vs_1_4 = (self.vf_1_4).to_string();
                                if self.get_contains_1(get_char_at(
                                    self.get_item_9(
                                        if let Some(result) =
                                            ((self.vf_1_4 - 1.0) as usize).checked_sub(1)
                                        {
                                            result
                                        } else {
                                            0
                                        },
                                    ),
                                    1.0,
                                )) || (self.vs_1_4.clone() == (2.0).to_string())
                                {
                                    self.vf_1_4 += 1.0;
                                    self.vs_1_4 = (self.vf_1_4).to_string();
                                };
                                let getin = ((self.get_leng_9() - 1.0) as i32 - 1) as usize;
                                let itemin = &*(if 2_usize
                                    != (self
                                        .get_item_9(
                                            if let Some(result) =
                                                ((self.get_leng_9() - 1.0) as usize).checked_sub(1)
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
                                        .get_item_9(
                                            if let Some(result) =
                                                ((self.get_leng_9() - 1.0) as usize).checked_sub(1)
                                            {
                                                result
                                            } else {
                                                0
                                            },
                                        )
                                        .get(
                                            (2_i32 - 1) as usize
                                                ..=((self
                                                    .get_item_9(
                                                        if let Some(result) =
                                                            ((self.get_leng_9() - 1.0) as usize)
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
                                        self.get_item_9(
                                            if let Some(result) =
                                                ((self.get_leng_9() - 1.0) as usize).checked_sub(1)
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
                                    if let Some(item) = self.l_1_9.get_mut(getin) {
                                        *item = itemin.to_owned();
                                    };
                                };
                            };
                        };
                        foreachnum3 += 1;
                    }
                    self.vf_1_3 = foreachvarnum3;
                    self.vs_1_3 = foreachvarstr3;
                };
                let insertin = self
                    .get_item_9(
                        if let Some(result) = (self.vf_1_4 as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                    .to_owned()
                    + &get_char_at(self.vs_1_5.clone(), self.vf_1_2).to_owned();
                let insertin2 = (self.vf_1_4 as i32 - 1) as usize;
                if insertin2 != usize::MAX {
                    self.l_1_9.insert(insertin2, insertin);
                }
                let removein = ((self.vf_1_4 + 1.0) as i32 - 1) as usize;
                if removein != usize::MAX {
                    if removein < self.l_1_9.len() {
                        self.l_1_9.remove(removein);
                    }
                }
            }
            let getin = ((self.get_leng_9()) as i32 - 1) as usize;
            let itemin = &*(if 2_usize
                != (self
                    .get_item_9(
                        if let Some(result) = ((self.get_leng_9()) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                    .chars()
                    .count() as f64) as usize
            {
                if let Some(substring) = self
                    .get_item_9(
                        if let Some(result) = ((self.get_leng_9()) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                    .get(
                        (2_i32 - 1) as usize
                            ..=((self
                                .get_item_9(
                                    if let Some(result) =
                                        ((self.get_leng_9()) as usize).checked_sub(1)
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
                    self.get_item_9(
                        if let Some(result) = ((self.get_leng_9()) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    ),
                    2.0,
                )
            });
            if getin != usize::MAX {
                if let Some(item) = self.l_1_9.get_mut(getin) {
                    *item = itemin.to_owned();
                };
            };
            let foreachvarnum4 = self.vf_1_2;
            let foreachvarstr4 = (self.vs_1_2).clone();
            self.vs_1_2 = String::from("0");
            self.vf_1_2 = 0.0;
            let foreachto4 = (self.get_leng_9()) as usize;
            let mut foreachnum4 = 1;
            while foreachnum4 <= foreachto4 {
                self.vf_1_2 = foreachnum4 as f64;
                self.vs_1_2 = foreachnum4.to_string();
                let getin = (self.vf_1_2 as i32 - 1) as usize;
                let itemin = &*(self
                    .get_item_9(
                        if let Some(result) = (self.vf_1_2 as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                    .replace(" ", ""));
                if getin != usize::MAX {
                    if let Some(item) = self.l_1_9.get_mut(getin) {
                        *item = itemin.to_owned();
                    };
                };
                if self.get_item_9(
                    if let Some(result) = (self.vf_1_2 as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                ) == String::from("")
                {
                    let removein = (self.vf_1_2 as i32 - 1) as usize;
                    if removein != usize::MAX {
                        if removein < self.l_1_9.len() {
                            self.l_1_9.remove(removein);
                        }
                    }
                };
                foreachnum4 += 1;
            }
            self.vf_1_2 = foreachvarnum4;
            self.vs_1_2 = foreachvarstr4;
            self.vf_1_2 = 0.0;
            self.vs_1_2 = (0.0).to_string();
            let repeatto = (self.get_leng_9()) as usize;
            for _ in 0..repeatto {
                self.vf_1_2 += 1.0;
                self.vs_1_2 = (self.vf_1_2).to_string();
                if self.get_item_9(
                    if let Some(result) = (self.vf_1_2 as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                ) == String::from("-")
                {
                    if self.get_contains_1(self.get_item_9(
                        if let Some(result) = ((self.vf_1_2 - 1.0) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )) {
                        let insertin = (0.0).to_string();
                        let insertin2 = (self.vf_1_2 as i32 - 1) as usize;
                        if insertin2 != usize::MAX {
                            self.l_1_9.insert(insertin2, insertin);
                        }
                        self.vf_1_2 += 1.0;
                        self.vs_1_2 = (self.vf_1_2).to_string();
                    };
                    let getin = (self.vf_1_2 as i32 - 1) as usize;
                    let itemin = "+";
                    if getin != usize::MAX {
                        if let Some(item) = self.l_1_9.get_mut(getin) {
                            *item = itemin.to_owned();
                        };
                    };
                    let getin = ((self.vf_1_2 + 1.0) as i32 - 1) as usize;
                    let itemin = &*("-".to_owned()
                        + &self
                            .get_item_9(
                                if let Some(result) = ((self.vf_1_2 + 1.0) as usize).checked_sub(1)
                                {
                                    result
                                } else {
                                    0
                                },
                            )
                            .to_owned());
                    if getin != usize::MAX {
                        if let Some(item) = self.l_1_9.get_mut(getin) {
                            *item = itemin.to_owned();
                        };
                    };
                    self.vf_1_2 += 1.0;
                    self.vs_1_2 = (self.vf_1_2).to_string();
                };
            }
            self.vf_1_4 = 0.0;
            self.vs_1_4 = (0.0).to_string();
            self.vf_1_2 = 0.0;
            self.vs_1_2 = (0.0).to_string();
            self.vf_1_7 = 0.0;
            self.vs_1_7 = (0.0).to_string();
            let repeatto = (self.get_leng_9()) as usize;
            for _ in 0..repeatto {
                self.vf_1_2 += 1.0;
                self.vs_1_2 = (self.vf_1_2).to_string();
                if self.get_item_9(
                    if let Some(result) = (self.vf_1_2 as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                ) == String::from("(")
                {
                    self.vf_1_4 += 1.0;
                    self.vs_1_4 = (self.vf_1_4).to_string();
                    let removein = (self.vf_1_2 as i32 - 1) as usize;
                    if removein != usize::MAX {
                        if removein < self.l_1_9.len() {
                            self.l_1_9.remove(removein);
                        }
                    }
                    self.vf_1_2 += -1.0;
                    self.vs_1_2 = (self.vf_1_2).to_string();
                } else {
                    if self.get_item_9(
                        if let Some(result) = (self.vf_1_2 as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    ) == String::from(")")
                    {
                        self.vf_1_4 += -1.0;
                        self.vs_1_4 = (self.vf_1_4).to_string();
                        let removein = (self.vf_1_2 as i32 - 1) as usize;
                        if removein != usize::MAX {
                            if removein < self.l_1_9.len() {
                                self.l_1_9.remove(removein);
                            }
                        }
                    } else {
                        let getin = (self.vf_1_2 as i32 - 1) as usize;
                        let itemin = &*(repeat(" ", self.vf_1_4 as usize).to_owned()
                            + &self
                                .get_item_9(
                                    if let Some(result) = (self.vf_1_2 as usize).checked_sub(1) {
                                        result
                                    } else {
                                        0
                                    },
                                )
                                .to_owned());
                        if getin != usize::MAX {
                            if let Some(item) = self.l_1_9.get_mut(getin) {
                                *item = itemin.to_owned();
                            };
                        };
                    };
                };
            }
            self.l_1_10.clear();
            self.vf_1_4 = 0.0;
            self.vs_1_4 = (0.0).to_string();
            let foreachvarnum5 = self.vf_1_2;
            let foreachvarstr5 = (self.vs_1_2).clone();
            self.vs_1_2 = String::from("0");
            self.vf_1_2 = 0.0;
            let foreachto5 = (self.get_leng_9()) as usize;
            let mut foreachnum5 = 1;
            while foreachnum5 <= foreachto5 {
                self.vf_1_2 = foreachnum5 as f64;
                self.vs_1_2 = foreachnum5.to_string();
                if (self
                    .get_item_9(
                        if let Some(result) = (self.vf_1_2 as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                    .matches(" ")
                    .count() as f64)
                    > self.vf_1_4
                {
                    self.vf_1_4 += 1.0;
                    self.vs_1_4 = (self.vf_1_4).to_string();
                    let addin = self.vs_1_4.clone();
                    self.l_1_10.push(addin);
                } else {
                    if (self
                        .get_item_9(
                            if let Some(result) = (self.vf_1_2 as usize).checked_sub(1) {
                                result
                            } else {
                                0
                            },
                        )
                        .matches(" ")
                        .count() as f64)
                        < self.vf_1_4
                    {
                        self.vf_1_4 += -1.0;
                        self.vs_1_4 = (self.vf_1_4).to_string();
                        let addin = self.vs_1_4.clone();
                        self.l_1_10.push(addin);
                    } else {
                        let addin = String::from("");
                        self.l_1_10.push(addin);
                    };
                };
                foreachnum5 += 1;
            }
            self.vf_1_2 = foreachvarnum5;
            self.vs_1_2 = foreachvarstr5;
            let addin = (0.0).to_string();
            self.l_1_10.push(addin);
            self.vf_1_4 = 0.0;
            self.vs_1_4 = (0.0).to_string();
            let foreachvarnum6 = self.vf_1_2;
            let foreachvarstr6 = (self.vs_1_2).clone();
            self.vs_1_2 = String::from("0");
            self.vf_1_2 = 0.0;
            let foreachto6 = (self.get_leng_10()) as usize;
            let mut foreachnum6 = 1;
            while foreachnum6 <= foreachto6 {
                self.vf_1_2 = foreachnum6 as f64;
                self.vs_1_2 = foreachnum6.to_string();
                if get_f64_string(
                    self.get_item_10(
                        if let Some(result) = (self.vf_1_2 as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    )
                    .clone(),
                ) > self.vf_1_4
                {
                    self.vf_1_4 += 1.0;
                    self.vs_1_4 = (self.vf_1_4).to_string();
                };
                foreachnum6 += 1;
            }
            self.vf_1_2 = foreachvarnum6;
            self.vs_1_2 = foreachvarstr6;
            self.vf_1_7 = self.vf_1_4;
            self.vs_1_7 = self.vs_1_4.clone();
            let repeatto = self.vf_1_4 as usize;
            for _ in 0..repeatto {
                while !(!(self.get_contains_10(self.vs_1_7.clone()))) {
                    self.l_1_11.clear();
                    self.vf_1_2 = self.get_position_10(&*self.vs_1_7);
                    self.vs_1_2 = (self.get_position_10(&*self.vs_1_7)).to_string();
                    let repeatto = (self.get_position_10(&((self.vf_1_7 - 1.0).to_string()))
                        - self.get_position_10(&*self.vs_1_7))
                        as usize;
                    for _ in 0..repeatto {
                        let addin = self.get_item_9(
                            if let Some(result) = (self.vf_1_2 as usize).checked_sub(1) {
                                result
                            } else {
                                0
                            },
                        );
                        self.l_1_11.push(addin);
                        let removein = (self.vf_1_2 as i32 - 1) as usize;
                        if removein != usize::MAX {
                            if removein < self.l_1_9.len() {
                                self.l_1_9.remove(removein);
                            }
                        }
                        let removein = (self.vf_1_2 as i32 - 1) as usize;
                        if removein != usize::MAX {
                            if removein < self.l_1_10.len() {
                                self.l_1_10.remove(removein);
                            }
                        }
                    }
                    let foreachvarnum7 = self.vf_1_3;
                    let foreachvarstr7 = (self.vs_1_3).clone();
                    self.vs_1_3 = String::from("0");
                    self.vf_1_3 = 0.0;
                    let foreachto7 = (self.get_leng_11()) as usize;
                    let mut foreachnum7 = 1;
                    while foreachnum7 <= foreachto7 {
                        self.vf_1_3 = foreachnum7 as f64;
                        self.vs_1_3 = foreachnum7.to_string();
                        let getin = (self.vf_1_3 as i32 - 1) as usize;
                        let itemin = &*(self
                            .get_item_11(
                                if let Some(result) = (self.vf_1_3 as usize).checked_sub(1) {
                                    result
                                } else {
                                    0
                                },
                            )
                            .replace(" ", ""));
                        if getin != usize::MAX {
                            if let Some(item) = self.l_1_11.get_mut(getin) {
                                *item = itemin.to_owned();
                            };
                        };
                        if self.get_item_11(
                            if let Some(result) = (self.vf_1_3 as usize).checked_sub(1) {
                                result
                            } else {
                                0
                            },
                        ) == String::from("")
                        {
                            let removein = (self.vf_1_3 as i32 - 1) as usize;
                            if removein != usize::MAX {
                                if removein < self.l_1_11.len() {
                                    self.l_1_11.remove(removein);
                                }
                            }
                        };
                        foreachnum7 += 1;
                    }
                    self.vf_1_3 = foreachvarnum7;
                    self.vs_1_3 = foreachvarstr7;
                    self.proc4();
                    let insertin = String::from("");
                    let insertin2 = (self.vf_1_2 as i32 - 1) as usize;
                    if insertin2 != usize::MAX {
                        self.l_1_10.insert(insertin2, insertin);
                    }
                    let getin = ((self.get_position_10(&((self.vf_1_4 - 1.0).to_string()))) as i32
                        - 1) as usize;
                    let itemin = "";
                    if getin != usize::MAX {
                        if let Some(item) = self.l_1_10.get_mut(getin) {
                            *item = itemin.to_owned();
                        };
                    };
                    let insertin = self.vs_1_6.clone();
                    let insertin2 = (self.vf_1_2 as i32 - 1) as usize;
                    if insertin2 != usize::MAX {
                        self.l_1_9.insert(insertin2, insertin);
                    }
                }
                self.vf_1_7 += -1.0;
                self.vs_1_7 = (self.vf_1_7).to_string();
            }
            self.l_1_11.clear();
            let foreachvarnum8 = self.vf_1_2;
            let foreachvarstr8 = (self.vs_1_2).clone();
            self.vs_1_2 = String::from("0");
            self.vf_1_2 = 0.0;
            let foreachto8 = (self.get_leng_9()) as usize;
            let mut foreachnum8 = 1;
            while foreachnum8 <= foreachto8 {
                self.vf_1_2 = foreachnum8 as f64;
                self.vs_1_2 = foreachnum8.to_string();
                let addin = self.get_item_9(
                    if let Some(result) = (self.vf_1_2 as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                );
                self.l_1_11.push(addin);
                foreachnum8 += 1;
            }
            self.vf_1_2 = foreachvarnum8;
            self.vs_1_2 = foreachvarstr8;
            self.proc4();
            self.l_1_9.clear();
            return Arc::new(self.vs_1_6.clone());
        };
        Arc::new(self.vs_1_6.clone())
    }
}
pub fn calc<T: AsRef<str>>(expression: T) -> f64 {
    let mut init = Default::new();
    get_f64_string(Arc::as_ref(&init.proc5(expression.as_ref())))
}
