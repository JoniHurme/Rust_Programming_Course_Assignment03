
pub fn extract_severity(log_string: &str) -> &str{

    let slice_start = log_string.find("[");
    let slice_end = log_string.find("]");

    if slice_start.is_none() || slice_end.is_none() {
        // println!("{}", log_string);
        return log_string;
    }else {
        let sliced_log = &log_string[slice_start.unwrap() + 1..slice_end.unwrap()];
        // println!("Extracted Severity: {}", sliced_log);
        return sliced_log;
    }
}

pub fn append_alert(summary: &mut String, severity: &str, message: &str) {

    summary.push_str("[");
    summary.push_str(severity);
    summary.push_str("] ");
    summary.push_str(message);
    summary.push_str("\n");
    // println!("[{}] {}\n", severity, message);

    println!("{}", summary)

}

pub fn archive_log(mut payload: String, status_code: u32) -> String {

    payload.push_str(&format!(" [CODE: {}]", status_code));

    println!("{}", payload);
    // println!("Archived output: {} [Code {}]", payload, status_code);

    payload

}