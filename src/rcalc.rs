// By Lingba Saner 24125 🍥 v0.9-final
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::sync::Arc;
lazy_static! {
    static ref GLOBAL_LIST_1: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref GLOBAL_LIST_2: Mutex<Vec<String>> = Mutex::new(vec![]);
}
lazy_static! {
    static ref _1_NUM_3: Mutex<f64> = Mutex::new(3.0);
    static ref _1_STR_3: Mutex<String> = Mutex::new((3).to_string());
    static ref _1_NUM_4: Mutex<f64> = Mutex::new(17.0);
    static ref _1_STR_4: Mutex<String> = Mutex::new((17).to_string());
    static ref _1_NUM_5: Mutex<f64> = Mutex::new(0.0);
    static ref _1_STR_5: Mutex<String> = Mutex::new((0).to_string());
    static ref _1_NUM_6: Mutex<f64> = Mutex::new(0.0);
    static ref _1_STR_6: Mutex<String> = Mutex::new(("!1+13").to_string());
    static ref _1_NUM_7: Mutex<f64> = Mutex::new(14.0);
    static ref _1_STR_7: Mutex<String> = Mutex::new((14).to_string());
    static ref _1_NUM_8: Mutex<f64> = Mutex::new(0.0);
    static ref _1_STR_8: Mutex<String> = Mutex::new((0).to_string());
    static ref _1_NUM_9: Mutex<f64> = Mutex::new(1.0);
    static ref _1_STR_9: Mutex<String> = Mutex::new((1).to_string());
    static ref _1_LIST_10: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref _1_LIST_11: Mutex<Vec<String>> = Mutex::new(vec![]);
    static ref _1_LIST_12: Mutex<Vec<String>> = Mutex::new(vec![]);
}
fn get_f64_string<T: AsRef<str>>(toget: T) -> f64 {
    let num: Result<f64, _> = toget.as_ref().parse();
    match num {
        Ok(parsed_num) => parsed_num,
        Err(_) => 0.0,
    }
}

fn utilities_string_to_boolean<T: AsRef<str>>(input: T) -> bool {
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
fn get_1_str_7() -> String {
    let globalvar = _1_STR_7.lock().unwrap().clone();
    globalvar
}
fn get_contains_1_list_12(item: String) -> bool {
    _1_LIST_12.lock().unwrap().contains(&item)
}
fn get_position_1_list_12(value_to_find: String) -> f64 {
    if let Some(position) = _1_LIST_12
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
fn get_item_1_list_12(item: usize) -> String {
    if let Some(value) = _1_LIST_12.lock().unwrap().get(item) {
        value.to_string()
    } else {
        String::new()
    }
}
fn get_1_num_9() -> f64 {
    let globalvar = *_1_NUM_9.lock().unwrap();
    globalvar
}
fn get_leng_1_list_12() -> f64 {
    _1_LIST_12.lock().unwrap().len() as f64
}
fn get_1_num_6() -> f64 {
    let globalvar = *_1_NUM_6.lock().unwrap();
    globalvar
}
fn get_1_str_6() -> String {
    let globalvar = _1_STR_6.lock().unwrap().clone();
    globalvar
}
fn get_lengglobal_list_1() -> f64 {
    GLOBAL_LIST_1.lock().unwrap().len() as f64
}
fn get_1_num_3() -> f64 {
    let globalvar = *_1_NUM_3.lock().unwrap();
    globalvar
}
fn get_char_at<T: AsRef<str>>(s: T, index: f64) -> String {
    s.as_ref()
        .chars()
        .nth((index - 1.0) as usize)
        .map(|c| c.to_string())
        .unwrap_or_default()
}
fn get_1_num_4() -> f64 {
    let globalvar = *_1_NUM_4.lock().unwrap();
    globalvar
}
fn get_itemglobal_list_1(item: usize) -> String {
    if let Some(value) = GLOBAL_LIST_1.lock().unwrap().get(item) {
        value.to_string()
    } else {
        String::new()
    }
}
fn get_1_num_5() -> f64 {
    let globalvar = *_1_NUM_5.lock().unwrap();
    globalvar
}
fn get_1_str_5() -> String {
    let globalvar = _1_STR_5.lock().unwrap().clone();
    globalvar
}
fn get_1_num_8() -> f64 {
    let globalvar = *_1_NUM_8.lock().unwrap();
    globalvar
}
fn get_1_str_8() -> String {
    let globalvar = _1_STR_8.lock().unwrap().clone();
    globalvar
}
fn get_item_1_list_10(item: usize) -> String {
    if let Some(value) = _1_LIST_10.lock().unwrap().get(item) {
        value.to_string()
    } else {
        String::new()
    }
}
fn get_containsglobal_list_1(item: String) -> bool {
    GLOBAL_LIST_1.lock().unwrap().contains(&item)
}
fn get_leng_1_list_10() -> f64 {
    _1_LIST_10.lock().unwrap().len() as f64
}
fn strings_repeat(s: &str, times: usize) -> String {
    let mut result = String::new();
    for _ in 0..times {
        result.push_str(s);
    }
    result
}
fn get_leng_1_list_11() -> f64 {
    _1_LIST_11.lock().unwrap().len() as f64
}
fn get_item_1_list_11(item: usize) -> String {
    if let Some(value) = _1_LIST_11.lock().unwrap().get(item) {
        value.to_string()
    } else {
        String::new()
    }
}
fn get_contains_1_list_11(item: String) -> bool {
    _1_LIST_11.lock().unwrap().contains(&item)
}
fn get_position_1_list_11(value_to_find: String) -> f64 {
    if let Some(position) = _1_LIST_11
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
lazy_static! {
    static ref SENSING_ANSWER: Mutex<String> = Mutex::new(String::new());
}
fn proc1(pm_9b6ba8c0: String, pm_e810d60f: String, pm_f6b64c2b: String) {
    *_1_NUM_7.lock().unwrap() = 0.0;
    *_1_STR_7.lock().unwrap() = "0".to_string();
    let result = match &pm_e810d60f[..] {
        s if s.contains("==") && pm_9b6ba8c0 == pm_f6b64c2b => 1.0,
        s if s.contains("!=") && pm_9b6ba8c0 != pm_f6b64c2b => 1.0,
        s if s.contains("<")
            && get_f64_string(pm_9b6ba8c0.clone()) < get_f64_string(pm_f6b64c2b.clone()) =>
        {
            1.0
        }
        s if s.contains(">")
            && get_f64_string(pm_9b6ba8c0.clone()) > get_f64_string(pm_f6b64c2b.clone()) =>
        {
            1.0
        }
        s if s.contains("<=") && &pm_9b6ba8c0 <= &pm_f6b64c2b => 1.0,
        s if s.contains(">=") && &pm_9b6ba8c0 >= &pm_f6b64c2b => 1.0,
        s if s.contains("+") => {
            let value = get_f64_string(pm_9b6ba8c0) + get_f64_string(pm_f6b64c2b);
            *_1_NUM_7.lock().unwrap() = value;
            *_1_STR_7.lock().unwrap() = value.to_string();
            return;
        }
        s if s.contains("/") => {
            let value = get_f64_string(pm_9b6ba8c0) / get_f64_string(pm_f6b64c2b);
            *_1_NUM_7.lock().unwrap() = value;
            *_1_STR_7.lock().unwrap() = value.to_string();
            return;
        }
        s if s.contains("*") => {
            let value = get_f64_string(pm_9b6ba8c0) * get_f64_string(pm_f6b64c2b);
            *_1_NUM_7.lock().unwrap() = value;
            *_1_STR_7.lock().unwrap() = value.to_string();
            return;
        }
        s if s.contains("%") => {
            let value = get_f64_string(pm_9b6ba8c0) % get_f64_string(pm_f6b64c2b);
            *_1_NUM_7.lock().unwrap() = value;
            *_1_STR_7.lock().unwrap() = value.to_string();
            return;
        }
        s if s.contains("||") => {
            let value = utilities_string_to_boolean(pm_9b6ba8c0)
                || utilities_string_to_boolean(pm_f6b64c2b);
            *_1_NUM_7.lock().unwrap() = get_f64_string(if value { "1" } else { "0" });
            *_1_STR_7.lock().unwrap() = value.to_string();
            return;
        }
        s if s.contains("&&") => {
            let value = utilities_string_to_boolean(pm_9b6ba8c0)
                && utilities_string_to_boolean(pm_f6b64c2b);
            *_1_NUM_7.lock().unwrap() = get_f64_string(if value { "1" } else { "0" });
            *_1_STR_7.lock().unwrap() = value.to_string();
            return;
        }
        s if s.contains("..") => {
            let value = get_f64_string((pm_9b6ba8c0.clone() + &pm_f6b64c2b).clone());
            *_1_NUM_7.lock().unwrap() = value;
            *_1_STR_7.lock().unwrap() = (pm_9b6ba8c0 + &pm_f6b64c2b).to_string();
            return;
        }
        s if s == "" && pm_f6b64c2b == "" => {
            *_1_NUM_7.lock().unwrap() = get_f64_string(pm_9b6ba8c0.clone());
            *_1_STR_7.lock().unwrap() = pm_9b6ba8c0.clone();
            return;
        }
        _ => {
            *_1_NUM_7.lock().unwrap() = 0.0;
            *_1_STR_7.lock().unwrap() = "Error".to_string();
            return;
        }
    };
    *_1_NUM_7.lock().unwrap() = result;
    *_1_STR_7.lock().unwrap() = result.to_string();
}
fn proc2(pm_16: String) {
    while !(!(get_contains_1_list_12(pm_16.clone()))) {
        proc1(
            (get_item_1_list_12(
                if let Some(result) =
                    (((get_position_1_list_12(pm_16.clone())) - 1.0) as usize).checked_sub(1)
                {
                    result
                } else {
                    0
                },
            ))
            .to_string(),
            (get_item_1_list_12(
                if let Some(result) =
                    ((get_position_1_list_12(pm_16.clone())) as usize).checked_sub(1)
                {
                    result
                } else {
                    0
                },
            ))
            .to_string(),
            (get_item_1_list_12(
                if let Some(result) =
                    (((get_position_1_list_12(pm_16.clone())) + 1.0) as usize).checked_sub(1)
                {
                    result
                } else {
                    0
                },
            ))
            .to_string(),
        );
        *_1_NUM_9.lock().unwrap() = (get_position_1_list_12(pm_16.clone())) - 1.0;
        *_1_STR_9.lock().unwrap() = ((get_position_1_list_12(pm_16.clone())) - 1.0).to_string();
        let repeatto = (2.0) as usize;
        for _ in 0..repeatto {
            let removein = ((get_1_num_9()) as i32 - 1) as usize;
            if removein != usize::MAX {
                if removein < _1_LIST_12.lock().unwrap().len() {
                    _1_LIST_12.lock().unwrap().remove(removein);
                }
            }
        }
        let getin = ((get_1_num_9()) as i32 - 1) as usize;
        let itemin = get_1_str_7();
        if getin != usize::MAX {
            if let Some(item) = _1_LIST_12.lock().unwrap().get_mut(getin) {
                *item = itemin;
            };
        };
    }
}
fn proc3() {
    while !(!(get_contains_1_list_12((("*").to_string()).to_string()))
        && !(get_contains_1_list_12((("/").to_string()).to_string())))
    {
        if (get_position_1_list_12((("*").to_string()).to_string()))
            > (get_position_1_list_12((("/").to_string()).to_string()))
        {
            proc1(
                (get_item_1_list_12(
                    if let Some(result) =
                        (((get_position_1_list_12((("*").to_string()).to_string())) - 1.0) as usize)
                            .checked_sub(1)
                    {
                        result
                    } else {
                        0
                    },
                ))
                .to_string(),
                (get_item_1_list_12(
                    if let Some(result) = ((get_position_1_list_12((("*").to_string()).to_string()))
                        as usize)
                        .checked_sub(1)
                    {
                        result
                    } else {
                        0
                    },
                ))
                .to_string(),
                (get_item_1_list_12(
                    if let Some(result) =
                        (((get_position_1_list_12((("*").to_string()).to_string())) + 1.0) as usize)
                            .checked_sub(1)
                    {
                        result
                    } else {
                        0
                    },
                ))
                .to_string(),
            );
            *_1_NUM_9.lock().unwrap() =
                (get_position_1_list_12((("*").to_string()).to_string())) - 1.0;
            *_1_STR_9.lock().unwrap() =
                ((get_position_1_list_12((("*").to_string()).to_string())) - 1.0).to_string();
            let repeatto = (2.0) as usize;
            for _ in 0..repeatto {
                let removein = ((get_1_num_9()) as i32 - 1) as usize;
                if removein != usize::MAX {
                    if removein < _1_LIST_12.lock().unwrap().len() {
                        _1_LIST_12.lock().unwrap().remove(removein);
                    }
                }
            }
            let getin = ((get_1_num_9()) as i32 - 1) as usize;
            let itemin = get_1_str_7();
            if getin != usize::MAX {
                if let Some(item) = _1_LIST_12.lock().unwrap().get_mut(getin) {
                    *item = itemin;
                };
            };
        } else {
            proc1(
                (get_item_1_list_12(
                    if let Some(result) =
                        (((get_position_1_list_12((("/").to_string()).to_string())) - 1.0) as usize)
                            .checked_sub(1)
                    {
                        result
                    } else {
                        0
                    },
                ))
                .to_string(),
                (get_item_1_list_12(
                    if let Some(result) = ((get_position_1_list_12((("/").to_string()).to_string()))
                        as usize)
                        .checked_sub(1)
                    {
                        result
                    } else {
                        0
                    },
                ))
                .to_string(),
                (get_item_1_list_12(
                    if let Some(result) =
                        (((get_position_1_list_12((("/").to_string()).to_string())) + 1.0) as usize)
                            .checked_sub(1)
                    {
                        result
                    } else {
                        0
                    },
                ))
                .to_string(),
            );
            *_1_NUM_9.lock().unwrap() =
                (get_position_1_list_12((("/").to_string()).to_string())) - 1.0;
            *_1_STR_9.lock().unwrap() =
                ((get_position_1_list_12((("/").to_string()).to_string())) - 1.0).to_string();
            let repeatto = (2.0) as usize;
            for _ in 0..repeatto {
                let removein = ((get_1_num_9()) as i32 - 1) as usize;
                if removein != usize::MAX {
                    if removein < _1_LIST_12.lock().unwrap().len() {
                        _1_LIST_12.lock().unwrap().remove(removein);
                    }
                }
            }
            let getin = ((get_1_num_9()) as i32 - 1) as usize;
            let itemin = get_1_str_7();
            if getin != usize::MAX {
                if let Some(item) = _1_LIST_12.lock().unwrap().get_mut(getin) {
                    *item = itemin;
                };
            };
        };
    }
}
fn proc4() {
    *_1_NUM_7.lock().unwrap() = 0.0;
    *_1_STR_7.lock().unwrap() = (("false").to_string()).to_string();
    if (get_leng_1_list_12()).to_string() == ("1").to_string() {
        *_1_NUM_7.lock().unwrap() = get_f64_string(
            (get_item_1_list_12(if let Some(result) = ((1.0) as usize).checked_sub(1) {
                result
            } else {
                0
            }))
            .clone(),
        );
        *_1_STR_7.lock().unwrap() =
            (get_item_1_list_12(if let Some(result) = ((1.0) as usize).checked_sub(1) {
                result
            } else {
                0
            }))
            .to_string();
    } else {
        proc2(("..").to_string());
        proc2((">=").to_string());
        proc2(("<=").to_string());
        proc2(("==").to_string());
        proc2(("!=").to_string());
        proc2(("||").to_string());
        proc2(("&&").to_string());
        proc2(("^").to_string());
        proc2(("%").to_string());
        proc3();
        proc2(("+").to_string());
    };
}
fn proc5(pm_17: String) -> Arc<String> {
    *_1_NUM_6.lock().unwrap() = get_f64_string(pm_17.clone());
    *_1_STR_6.lock().unwrap() = pm_17.clone();
    *_1_NUM_5.lock().unwrap() = 0.0;
    *_1_STR_5.lock().unwrap() = ("0").to_string();
    let foreachvarnum2 = *_1_NUM_3.lock().unwrap();
    let foreachvarstr2 = (*_1_STR_3.lock().unwrap()).clone();
    *_1_STR_3.lock().unwrap() = String::from("0");
    *_1_NUM_3.lock().unwrap() = 0.0;
    let foreachto2 = (get_1_str_6().chars().count() as f64) as usize;
    let mut foreachnum2 = 1;
    while foreachnum2 <= foreachto2 {
        *_1_NUM_3.lock().unwrap() = foreachnum2 as f64;
        *_1_STR_3.lock().unwrap() = foreachnum2.to_string();
        let foreachvarnum1 = *_1_NUM_4.lock().unwrap();
        let foreachvarstr1 = (*_1_STR_4.lock().unwrap()).clone();
        *_1_STR_4.lock().unwrap() = String::from("0");
        *_1_NUM_4.lock().unwrap() = 0.0;
        let foreachto1 = (get_lengglobal_list_1()) as usize;
        let mut foreachnum1 = 1;
        while foreachnum1 <= foreachto1 {
            *_1_NUM_4.lock().unwrap() = foreachnum1 as f64;
            *_1_STR_4.lock().unwrap() = foreachnum1.to_string();
            if get_char_at(get_1_str_6(), get_1_num_3())
                == (get_itemglobal_list_1(
                    if let Some(result) = ((get_1_num_4()) as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                ))
                .to_string()
            {
                *_1_NUM_5.lock().unwrap() += 1.0;
                *_1_STR_5.lock().unwrap() = (*_1_NUM_5.lock().unwrap()).to_string();
            };
            foreachnum1 += 1;
        }
        *_1_NUM_4.lock().unwrap() = foreachvarnum1;
        *_1_STR_4.lock().unwrap() = foreachvarstr1;
        foreachnum2 += 1;
    }
    *_1_NUM_3.lock().unwrap() = foreachvarnum2;
    *_1_STR_3.lock().unwrap() = foreachvarstr2;
    if get_1_str_5() == ("0").to_string() {
        *_1_NUM_7.lock().unwrap() = get_1_num_6();
        *_1_STR_7.lock().unwrap() = get_1_str_6();
    } else {
        if !(get_char_at(get_1_str_6(), 1.0) == (("-").to_string()).to_string())
            && !(get_char_at(get_1_str_6(), 1.0) == (("!").to_string()).to_string())
        {
            *_1_NUM_6.lock().unwrap() =
                get_f64_string(("!".to_owned() + &get_1_str_6().to_owned()).clone());
            *_1_STR_6.lock().unwrap() = ("!".to_owned() + &get_1_str_6().to_owned()).to_string();
        };
        *_1_NUM_3.lock().unwrap() = 0.0;
        *_1_STR_3.lock().unwrap() = ("0").to_string();
        *_1_NUM_5.lock().unwrap() = 1.0;
        *_1_STR_5.lock().unwrap() = ("1").to_string();
        *_1_NUM_8.lock().unwrap() = 0.0;
        *_1_STR_8.lock().unwrap() = ("0").to_string();
        _1_LIST_10.lock().unwrap().clear();
        let repeatto = (get_1_str_6().chars().count() as f64) as usize;
        for _ in 0..repeatto {
            *_1_NUM_3.lock().unwrap() += 1.0;
            *_1_STR_3.lock().unwrap() = (*_1_NUM_3.lock().unwrap()).to_string();
            if (get_char_at(get_1_str_6(), get_1_num_3()) == (("\"").to_string()).to_string())
                || (get_char_at(get_1_str_6(), get_1_num_3()) == (("'").to_string()).to_string())
            {
                if get_1_str_8() == ("0").to_string() {
                    *_1_NUM_8.lock().unwrap() += 1.0;
                    *_1_STR_8.lock().unwrap() = (*_1_NUM_8.lock().unwrap()).to_string();
                } else {
                    *_1_NUM_8.lock().unwrap() += -1.0;
                    *_1_STR_8.lock().unwrap() = (*_1_NUM_8.lock().unwrap()).to_string();
                };
            };
            if get_1_str_8() == ("0").to_string() {
                let foreachvarnum3 = *_1_NUM_4.lock().unwrap();
                let foreachvarstr3 = (*_1_STR_4.lock().unwrap()).clone();
                *_1_STR_4.lock().unwrap() = String::from("0");
                *_1_NUM_4.lock().unwrap() = 0.0;
                let foreachto3 = (get_lengglobal_list_1()) as usize;
                let mut foreachnum3 = 1;
                while foreachnum3 <= foreachto3 {
                    *_1_NUM_4.lock().unwrap() = foreachnum3 as f64;
                    *_1_STR_4.lock().unwrap() = foreachnum3.to_string();
                    if (get_char_at(get_1_str_6(), get_1_num_3()).to_owned()
                        + &get_char_at(get_1_str_6(), get_1_num_3() + 1.0).to_owned())
                        .to_string()
                        == (get_itemglobal_list_1(
                            if let Some(result) = ((get_1_num_4()) as usize).checked_sub(1) {
                                result
                            } else {
                                0
                            },
                        ))
                        .to_string()
                    {
                        let addin = (get_char_at(get_1_str_6(), get_1_num_3()).to_owned()
                            + &get_char_at(get_1_str_6(), get_1_num_3() + 1.0).to_owned())
                            .to_string();
                        _1_LIST_10.lock().unwrap().push(addin);
                        *_1_NUM_5.lock().unwrap() += 1.0;
                        *_1_STR_5.lock().unwrap() = (*_1_NUM_5.lock().unwrap()).to_string();
                        *_1_NUM_3.lock().unwrap() += 1.0;
                        *_1_STR_3.lock().unwrap() = (*_1_NUM_3.lock().unwrap()).to_string();
                        if (get_containsglobal_list_1(
                            (get_char_at(
                                get_item_1_list_10(
                                    if let Some(result) =
                                        ((get_1_num_5() - 1.0) as usize).checked_sub(1)
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
                                    get_item_1_list_10(
                                        if let Some(result) =
                                            ((get_1_num_5() - 1.0) as usize).checked_sub(1)
                                        {
                                            result
                                        } else {
                                            0
                                        },
                                    ),
                                    2.0,
                                )
                                .to_owned())
                                .to_string(),
                        )) || (get_1_str_5() == ("2").to_string())
                        {
                            *_1_NUM_5.lock().unwrap() += 1.0;
                            *_1_STR_5.lock().unwrap() = (*_1_NUM_5.lock().unwrap()).to_string();
                        };
                        let getin = (((get_leng_1_list_10()) - 1.0) as i32 - 1) as usize;
                        let itemin = if (2.0) as usize
                            != ((get_item_1_list_10(
                                if let Some(result) =
                                    (((get_leng_1_list_10()) - 1.0) as usize).checked_sub(1)
                                {
                                    result
                                } else {
                                    0
                                },
                            ))
                            .chars()
                            .count() as f64) as usize
                        {
                            if let Some(substring) = (get_item_1_list_10(
                                if let Some(result) =
                                    (((get_leng_1_list_10()) - 1.0) as usize).checked_sub(1)
                                {
                                    result
                                } else {
                                    0
                                },
                            ))
                            .get(
                                ((2.0) as i32 - 1) as usize
                                    ..=(((get_item_1_list_10(
                                        if let Some(result) =
                                            (((get_leng_1_list_10()) - 1.0) as usize).checked_sub(1)
                                        {
                                            result
                                        } else {
                                            0
                                        },
                                    ))
                                    .chars()
                                    .count() as f64) as i32
                                        - 1) as usize,
                            ) {
                                substring.to_string()
                            } else {
                                String::new()
                            }
                        } else {
                            get_char_at(
                                get_item_1_list_10(
                                    if let Some(result) =
                                        (((get_leng_1_list_10()) - 1.0) as usize).checked_sub(1)
                                    {
                                        result
                                    } else {
                                        0
                                    },
                                ),
                                2.0,
                            )
                        };
                        if getin != usize::MAX {
                            if let Some(item) = _1_LIST_10.lock().unwrap().get_mut(getin) {
                                *item = itemin;
                            };
                        };
                    } else {
                        if get_char_at(get_1_str_6(), get_1_num_3())
                            == (get_itemglobal_list_1(
                                if let Some(result) = ((get_1_num_4()) as usize).checked_sub(1) {
                                    result
                                } else {
                                    0
                                },
                            ))
                            .to_string()
                        {
                            let addin = get_char_at(get_1_str_6(), get_1_num_3());
                            _1_LIST_10.lock().unwrap().push(addin);
                            *_1_NUM_5.lock().unwrap() += 1.0;
                            *_1_STR_5.lock().unwrap() = (*_1_NUM_5.lock().unwrap()).to_string();
                            if (get_containsglobal_list_1(get_char_at(
                                get_item_1_list_10(
                                    if let Some(result) =
                                        ((get_1_num_5() - 1.0) as usize).checked_sub(1)
                                    {
                                        result
                                    } else {
                                        0
                                    },
                                ),
                                1.0,
                            ))) || (get_1_str_5() == ("2").to_string())
                            {
                                *_1_NUM_5.lock().unwrap() += 1.0;
                                *_1_STR_5.lock().unwrap() = (*_1_NUM_5.lock().unwrap()).to_string();
                            };
                            let getin = (((get_leng_1_list_10()) - 1.0) as i32 - 1) as usize;
                            let itemin = if (2.0) as usize
                                != ((get_item_1_list_10(
                                    if let Some(result) =
                                        (((get_leng_1_list_10()) - 1.0) as usize).checked_sub(1)
                                    {
                                        result
                                    } else {
                                        0
                                    },
                                ))
                                .chars()
                                .count() as f64) as usize
                            {
                                if let Some(substring) = (get_item_1_list_10(
                                    if let Some(result) =
                                        (((get_leng_1_list_10()) - 1.0) as usize).checked_sub(1)
                                    {
                                        result
                                    } else {
                                        0
                                    },
                                ))
                                .get(
                                    ((2.0) as i32 - 1) as usize
                                        ..=(((get_item_1_list_10(
                                            if let Some(result) = (((get_leng_1_list_10()) - 1.0)
                                                as usize)
                                                .checked_sub(1)
                                            {
                                                result
                                            } else {
                                                0
                                            },
                                        ))
                                        .chars()
                                        .count()
                                            as f64)
                                            as i32
                                            - 1)
                                            as usize,
                                ) {
                                    substring.to_string()
                                } else {
                                    String::new()
                                }
                            } else {
                                get_char_at(
                                    get_item_1_list_10(
                                        if let Some(result) =
                                            (((get_leng_1_list_10()) - 1.0) as usize).checked_sub(1)
                                        {
                                            result
                                        } else {
                                            0
                                        },
                                    ),
                                    2.0,
                                )
                            };
                            if getin != usize::MAX {
                                if let Some(item) = _1_LIST_10.lock().unwrap().get_mut(getin) {
                                    *item = itemin;
                                };
                            };
                        };
                    };
                    foreachnum3 += 1;
                }
                *_1_NUM_4.lock().unwrap() = foreachvarnum3;
                *_1_STR_4.lock().unwrap() = foreachvarstr3;
            };
            let insertin = ((get_item_1_list_10(
                if let Some(result) = ((get_1_num_5()) as usize).checked_sub(1) {
                    result
                } else {
                    0
                },
            ))
            .to_owned()
                + &get_char_at(get_1_str_6(), get_1_num_3()).to_owned())
                .to_string();
            let insertin2 = ((get_1_num_5()) as i32 - 1) as usize;
            if insertin2 != usize::MAX {
                _1_LIST_10.lock().unwrap().insert(insertin2, insertin);
            }
            let removein = ((get_1_num_5() + 1.0) as i32 - 1) as usize;
            if removein != usize::MAX {
                if removein < _1_LIST_10.lock().unwrap().len() {
                    _1_LIST_10.lock().unwrap().remove(removein);
                }
            }
        }
        let getin = ((get_leng_1_list_10()) as i32 - 1) as usize;
        let itemin = if (2.0) as usize
            != ((get_item_1_list_10(
                if let Some(result) = ((get_leng_1_list_10()) as usize).checked_sub(1) {
                    result
                } else {
                    0
                },
            ))
            .chars()
            .count() as f64) as usize
        {
            if let Some(substring) = (get_item_1_list_10(
                if let Some(result) = ((get_leng_1_list_10()) as usize).checked_sub(1) {
                    result
                } else {
                    0
                },
            ))
            .get(
                ((2.0) as i32 - 1) as usize
                    ..=(((get_item_1_list_10(
                        if let Some(result) = ((get_leng_1_list_10()) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    ))
                    .chars()
                    .count() as f64) as i32
                        - 1) as usize,
            ) {
                substring.to_string()
            } else {
                String::new()
            }
        } else {
            get_char_at(
                get_item_1_list_10(
                    if let Some(result) = ((get_leng_1_list_10()) as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                ),
                2.0,
            )
        };
        if getin != usize::MAX {
            if let Some(item) = _1_LIST_10.lock().unwrap().get_mut(getin) {
                *item = itemin;
            };
        };
        let foreachvarnum4 = *_1_NUM_3.lock().unwrap();
        let foreachvarstr4 = (*_1_STR_3.lock().unwrap()).clone();
        *_1_STR_3.lock().unwrap() = String::from("0");
        *_1_NUM_3.lock().unwrap() = 0.0;
        let foreachto4 = (get_leng_1_list_10()) as usize;
        let mut foreachnum4 = 1;
        while foreachnum4 <= foreachto4 {
            *_1_NUM_3.lock().unwrap() = foreachnum4 as f64;
            *_1_STR_3.lock().unwrap() = foreachnum4.to_string();
            let getin = ((get_1_num_3()) as i32 - 1) as usize;
            let itemin = ((get_item_1_list_10(
                if let Some(result) = ((get_1_num_3()) as usize).checked_sub(1) {
                    result
                } else {
                    0
                },
            ))
            .replace(" ", ""))
            .to_string();
            if getin != usize::MAX {
                if let Some(item) = _1_LIST_10.lock().unwrap().get_mut(getin) {
                    *item = itemin;
                };
            };
            if (get_item_1_list_10(
                if let Some(result) = ((get_1_num_3()) as usize).checked_sub(1) {
                    result
                } else {
                    0
                },
            ))
            .to_string()
                == (("").to_string()).to_string()
            {
                let removein = ((get_1_num_3()) as i32 - 1) as usize;
                if removein != usize::MAX {
                    if removein < _1_LIST_10.lock().unwrap().len() {
                        _1_LIST_10.lock().unwrap().remove(removein);
                    }
                }
            };
            foreachnum4 += 1;
        }
        *_1_NUM_3.lock().unwrap() = foreachvarnum4;
        *_1_STR_3.lock().unwrap() = foreachvarstr4;
        *_1_NUM_3.lock().unwrap() = 0.0;
        *_1_STR_3.lock().unwrap() = ("0").to_string();
        let repeatto = (get_leng_1_list_10()) as usize;
        for _ in 0..repeatto {
            *_1_NUM_3.lock().unwrap() += 1.0;
            *_1_STR_3.lock().unwrap() = (*_1_NUM_3.lock().unwrap()).to_string();
            if (get_item_1_list_10(
                if let Some(result) = ((get_1_num_3()) as usize).checked_sub(1) {
                    result
                } else {
                    0
                },
            ))
            .to_string()
                == (("-").to_string()).to_string()
            {
                if get_containsglobal_list_1(
                    (get_item_1_list_10(
                        if let Some(result) = ((get_1_num_3() - 1.0) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    ))
                    .to_string(),
                ) {
                    let insertin = ("0").to_string();
                    let insertin2 = ((get_1_num_3()) as i32 - 1) as usize;
                    if insertin2 != usize::MAX {
                        _1_LIST_10.lock().unwrap().insert(insertin2, insertin);
                    }
                    *_1_NUM_3.lock().unwrap() += 1.0;
                    *_1_STR_3.lock().unwrap() = (*_1_NUM_3.lock().unwrap()).to_string();
                };
                let getin = ((get_1_num_3()) as i32 - 1) as usize;
                let itemin = (("+").to_string()).to_string();
                if getin != usize::MAX {
                    if let Some(item) = _1_LIST_10.lock().unwrap().get_mut(getin) {
                        *item = itemin;
                    };
                };
                let getin = ((get_1_num_3() + 1.0) as i32 - 1) as usize;
                let itemin = ("-".to_owned()
                    + &(get_item_1_list_10(
                        if let Some(result) = ((get_1_num_3() + 1.0) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    ))
                    .to_owned())
                    .to_string();
                if getin != usize::MAX {
                    if let Some(item) = _1_LIST_10.lock().unwrap().get_mut(getin) {
                        *item = itemin;
                    };
                };
                *_1_NUM_3.lock().unwrap() += 1.0;
                *_1_STR_3.lock().unwrap() = (*_1_NUM_3.lock().unwrap()).to_string();
            };
        }
        *_1_NUM_5.lock().unwrap() = 0.0;
        *_1_STR_5.lock().unwrap() = ("0").to_string();
        *_1_NUM_3.lock().unwrap() = 0.0;
        *_1_STR_3.lock().unwrap() = ("0").to_string();
        *_1_NUM_8.lock().unwrap() = 0.0;
        *_1_STR_8.lock().unwrap() = ("0").to_string();
        let repeatto = (get_leng_1_list_10()) as usize;
        for _ in 0..repeatto {
            *_1_NUM_3.lock().unwrap() += 1.0;
            *_1_STR_3.lock().unwrap() = (*_1_NUM_3.lock().unwrap()).to_string();
            if (get_item_1_list_10(
                if let Some(result) = ((get_1_num_3()) as usize).checked_sub(1) {
                    result
                } else {
                    0
                },
            ))
            .to_string()
                == (("(").to_string()).to_string()
            {
                *_1_NUM_5.lock().unwrap() += 1.0;
                *_1_STR_5.lock().unwrap() = (*_1_NUM_5.lock().unwrap()).to_string();
                let removein = ((get_1_num_3()) as i32 - 1) as usize;
                if removein != usize::MAX {
                    if removein < _1_LIST_10.lock().unwrap().len() {
                        _1_LIST_10.lock().unwrap().remove(removein);
                    }
                }
                *_1_NUM_3.lock().unwrap() += -1.0;
                *_1_STR_3.lock().unwrap() = (*_1_NUM_3.lock().unwrap()).to_string();
            } else {
                if (get_item_1_list_10(
                    if let Some(result) = ((get_1_num_3()) as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                ))
                .to_string()
                    == ((")").to_string()).to_string()
                {
                    *_1_NUM_5.lock().unwrap() += -1.0;
                    *_1_STR_5.lock().unwrap() = (*_1_NUM_5.lock().unwrap()).to_string();
                    let removein = ((get_1_num_3()) as i32 - 1) as usize;
                    if removein != usize::MAX {
                        if removein < _1_LIST_10.lock().unwrap().len() {
                            _1_LIST_10.lock().unwrap().remove(removein);
                        }
                    }
                } else {
                    let getin = ((get_1_num_3()) as i32 - 1) as usize;
                    let itemin = (strings_repeat(" ", (get_1_num_5()) as usize).to_owned()
                        + &(get_item_1_list_10(
                            if let Some(result) = ((get_1_num_3()) as usize).checked_sub(1) {
                                result
                            } else {
                                0
                            },
                        ))
                        .to_owned())
                        .to_string();
                    if getin != usize::MAX {
                        if let Some(item) = _1_LIST_10.lock().unwrap().get_mut(getin) {
                            *item = itemin;
                        };
                    };
                };
            };
        }
        _1_LIST_11.lock().unwrap().clear();
        *_1_NUM_5.lock().unwrap() = 0.0;
        *_1_STR_5.lock().unwrap() = ("0").to_string();
        let foreachvarnum5 = *_1_NUM_3.lock().unwrap();
        let foreachvarstr5 = (*_1_STR_3.lock().unwrap()).clone();
        *_1_STR_3.lock().unwrap() = String::from("0");
        *_1_NUM_3.lock().unwrap() = 0.0;
        let foreachto5 = (get_leng_1_list_10()) as usize;
        let mut foreachnum5 = 1;
        while foreachnum5 <= foreachto5 {
            *_1_NUM_3.lock().unwrap() = foreachnum5 as f64;
            *_1_STR_3.lock().unwrap() = foreachnum5.to_string();
            if (((get_item_1_list_10(
                if let Some(result) = ((get_1_num_3()) as usize).checked_sub(1) {
                    result
                } else {
                    0
                },
            ))
            .matches(" ")
            .count()) as f64)
                > get_1_num_5()
            {
                *_1_NUM_5.lock().unwrap() += 1.0;
                *_1_STR_5.lock().unwrap() = (*_1_NUM_5.lock().unwrap()).to_string();
                let addin = get_1_str_5();
                _1_LIST_11.lock().unwrap().push(addin);
            } else {
                if (((get_item_1_list_10(
                    if let Some(result) = ((get_1_num_3()) as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                ))
                .matches(" ")
                .count()) as f64)
                    < get_1_num_5()
                {
                    *_1_NUM_5.lock().unwrap() += -1.0;
                    *_1_STR_5.lock().unwrap() = (*_1_NUM_5.lock().unwrap()).to_string();
                    let addin = get_1_str_5();
                    _1_LIST_11.lock().unwrap().push(addin);
                } else {
                    let addin = (("").to_string()).to_string();
                    _1_LIST_11.lock().unwrap().push(addin);
                };
            };
            foreachnum5 += 1;
        }
        *_1_NUM_3.lock().unwrap() = foreachvarnum5;
        *_1_STR_3.lock().unwrap() = foreachvarstr5;
        let addin = ("0").to_string();
        _1_LIST_11.lock().unwrap().push(addin);
        *_1_NUM_5.lock().unwrap() = 0.0;
        *_1_STR_5.lock().unwrap() = ("0").to_string();
        let foreachvarnum6 = *_1_NUM_3.lock().unwrap();
        let foreachvarstr6 = (*_1_STR_3.lock().unwrap()).clone();
        *_1_STR_3.lock().unwrap() = String::from("0");
        *_1_NUM_3.lock().unwrap() = 0.0;
        let foreachto6 = (get_leng_1_list_11()) as usize;
        let mut foreachnum6 = 1;
        while foreachnum6 <= foreachto6 {
            *_1_NUM_3.lock().unwrap() = foreachnum6 as f64;
            *_1_STR_3.lock().unwrap() = foreachnum6.to_string();
            if get_f64_string(
                (get_item_1_list_11(
                    if let Some(result) = ((get_1_num_3()) as usize).checked_sub(1) {
                        result
                    } else {
                        0
                    },
                ))
                .clone(),
            ) > get_1_num_5()
            {
                *_1_NUM_5.lock().unwrap() += 1.0;
                *_1_STR_5.lock().unwrap() = (*_1_NUM_5.lock().unwrap()).to_string();
            };
            foreachnum6 += 1;
        }
        *_1_NUM_3.lock().unwrap() = foreachvarnum6;
        *_1_STR_3.lock().unwrap() = foreachvarstr6;
        *_1_NUM_8.lock().unwrap() = get_1_num_5();
        *_1_STR_8.lock().unwrap() = get_1_str_5();
        let repeatto = (get_1_num_5()) as usize;
        for _ in 0..repeatto {
            while !(!(get_contains_1_list_11(get_1_str_8()))) {
                _1_LIST_12.lock().unwrap().clear();
                *_1_NUM_3.lock().unwrap() = get_position_1_list_11(get_1_str_8());
                *_1_STR_3.lock().unwrap() = (get_position_1_list_11(get_1_str_8())).to_string();
                let repeatto = ((get_position_1_list_11((get_1_num_8() - 1.0).to_string()))
                    - (get_position_1_list_11(get_1_str_8())))
                    as usize;
                for _ in 0..repeatto {
                    let addin = (get_item_1_list_10(
                        if let Some(result) = ((get_1_num_3()) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    ))
                    .to_string();
                    _1_LIST_12.lock().unwrap().push(addin);
                    let removein = ((get_1_num_3()) as i32 - 1) as usize;
                    if removein != usize::MAX {
                        if removein < _1_LIST_10.lock().unwrap().len() {
                            _1_LIST_10.lock().unwrap().remove(removein);
                        }
                    }
                    let removein = ((get_1_num_3()) as i32 - 1) as usize;
                    if removein != usize::MAX {
                        if removein < _1_LIST_11.lock().unwrap().len() {
                            _1_LIST_11.lock().unwrap().remove(removein);
                        }
                    }
                }
                let foreachvarnum7 = *_1_NUM_4.lock().unwrap();
                let foreachvarstr7 = (*_1_STR_4.lock().unwrap()).clone();
                *_1_STR_4.lock().unwrap() = String::from("0");
                *_1_NUM_4.lock().unwrap() = 0.0;
                let foreachto7 = (get_leng_1_list_12()) as usize;
                let mut foreachnum7 = 1;
                while foreachnum7 <= foreachto7 {
                    *_1_NUM_4.lock().unwrap() = foreachnum7 as f64;
                    *_1_STR_4.lock().unwrap() = foreachnum7.to_string();
                    let getin = ((get_1_num_4()) as i32 - 1) as usize;
                    let itemin = ((get_item_1_list_12(
                        if let Some(result) = ((get_1_num_4()) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    ))
                    .replace(" ", ""))
                    .to_string();
                    if getin != usize::MAX {
                        if let Some(item) = _1_LIST_12.lock().unwrap().get_mut(getin) {
                            *item = itemin;
                        };
                    };
                    if (get_item_1_list_12(
                        if let Some(result) = ((get_1_num_4()) as usize).checked_sub(1) {
                            result
                        } else {
                            0
                        },
                    ))
                    .to_string()
                        == (("").to_string()).to_string()
                    {
                        let removein = ((get_1_num_4()) as i32 - 1) as usize;
                        if removein != usize::MAX {
                            if removein < _1_LIST_12.lock().unwrap().len() {
                                _1_LIST_12.lock().unwrap().remove(removein);
                            }
                        }
                    };
                    foreachnum7 += 1;
                }
                *_1_NUM_4.lock().unwrap() = foreachvarnum7;
                *_1_STR_4.lock().unwrap() = foreachvarstr7;
                proc4();
                let insertin = (("").to_string()).to_string();
                let insertin2 = ((get_1_num_3()) as i32 - 1) as usize;
                if insertin2 != usize::MAX {
                    _1_LIST_11.lock().unwrap().insert(insertin2, insertin);
                }
                let getin = ((get_position_1_list_11((get_1_num_5() - 1.0).to_string())) as i32 - 1)
                    as usize;
                let itemin = (("").to_string()).to_string();
                if getin != usize::MAX {
                    if let Some(item) = _1_LIST_11.lock().unwrap().get_mut(getin) {
                        *item = itemin;
                    };
                };
                let insertin = get_1_str_7();
                let insertin2 = ((get_1_num_3()) as i32 - 1) as usize;
                if insertin2 != usize::MAX {
                    _1_LIST_10.lock().unwrap().insert(insertin2, insertin);
                }
            }
            *_1_NUM_8.lock().unwrap() += -1.0;
            *_1_STR_8.lock().unwrap() = (*_1_NUM_8.lock().unwrap()).to_string();
        }
        _1_LIST_12.lock().unwrap().clear();
        let foreachvarnum8 = *_1_NUM_3.lock().unwrap();
        let foreachvarstr8 = (*_1_STR_3.lock().unwrap()).clone();
        *_1_STR_3.lock().unwrap() = String::from("0");
        *_1_NUM_3.lock().unwrap() = 0.0;
        let foreachto8 = (get_leng_1_list_10()) as usize;
        let mut foreachnum8 = 1;
        while foreachnum8 <= foreachto8 {
            *_1_NUM_3.lock().unwrap() = foreachnum8 as f64;
            *_1_STR_3.lock().unwrap() = foreachnum8.to_string();
            let addin = (get_item_1_list_10(
                if let Some(result) = ((get_1_num_3()) as usize).checked_sub(1) {
                    result
                } else {
                    0
                },
            ))
            .to_string();
            _1_LIST_12.lock().unwrap().push(addin);
            foreachnum8 += 1;
        }
        *_1_NUM_3.lock().unwrap() = foreachvarnum8;
        *_1_STR_3.lock().unwrap() = foreachvarstr8;
        proc4();
        _1_LIST_10.lock().unwrap().clear();
        return Arc::new(get_1_str_7().to_string());
    };
    return Arc::new(get_1_str_7().to_string());
}
/// Computational expression
pub fn calc<T: AsRef<str>>(expression: T) -> f64 {
    *GLOBAL_LIST_1.lock().unwrap() = [
        "==", ">=", "<=", "!=", ">", "<", "+", "-", "*", "/", "%", "&&", "||", "..", "(", ")",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    get_f64_string(Arc::as_ref(&proc5(expression.as_ref().to_owned())))
}
