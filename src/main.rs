mod functions;

fn main() {


    let log_string_correct = "[WARN] Low disk space";
    let error_code: u32 = 404;

    let mut summary = String::from("System Log Summary\n");
    let severity = functions::extract_severity(log_string_correct);
    println!("Extracted severity: {}\n", severity);
    let message = "Disk space low";

    let payload = String::from("Archived output:");
    functions::append_alert(&mut summary, severity, message);

    functions::archive_log(payload, error_code);

}
