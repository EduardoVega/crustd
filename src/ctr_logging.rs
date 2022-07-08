use anyhow::{Result,anyhow};

pub fn configure_log_drivers(
    log_drivers: Vec<String>,
    log_size_max: i64, 
    cid: String,
    name: String,
    tag: String){

    
    for log_driver in log_drivers.iter() {
        parse_log_drivers(log_driver);
    }
}

/*
* parse_log_path branches on log driver type the user inputted.
* log_config will either be a ':' delimited string containing:
* <DRIVER_NAME>:<PATH_NAME> or <PATH_NAME>
* in the case of no colon, the driver will be kubernetes-log-file,
* in the case the log driver is 'journald', the <PATH_NAME> is ignored.
* exits with error if <DRIVER_NAME> isn't 'journald' or 'kubernetes-log-file'
*/
fn parse_log_drivers(log_driver: &String) -> Result<()> {
    let driver: Option<&str>;
    let path: Option<&str>;
    
    let v: Vec<&str> = log_driver.split(":").collect();

    match v.len() {
        2 => {
            driver = Some(v[0]);
            path = Some(v[1]);
        },
        1 => {
            driver = Some(v[0]);
            path = None;
        },
        _ => {
            return Err(anyhow!("log-path can not be empty"))
        }
    }

    // // :none is not the same as none, nor is :journald the same as journald
	// // we check the delim here though, because we DO want to match "none" as the none driver
	// if (path == NULL && delim == log_config) {
	// 	path = driver;
	// 	driver = (char *)K8S_FILE_STRING;
	// }

	// if (!strcmp(driver, "off") || !strcmp(driver, "null") || !strcmp(driver, "none")) {
	// 	// no-op, this means things like --log-driver journald --log-driver none will still log to journald.
	// 	return;
	// }

	// if (!strcmp(driver, "passthrough")) {
	// 	if (isatty(STDIN_FILENO) || isatty(STDOUT_FILENO) || isatty(STDERR_FILENO))
	// 		nexitf("cannot use a tty with passthrough logging mode to prevent attacks via TIOCSTI");

	// 	use_logging_passthrough = TRUE;
	// 	return;
	// }

	// if (!strcmp(driver, JOURNALD_FILE_STRING)) {
	// 	use_journald_logging = TRUE;
	// 	return;
	// }

	// // Driver is k8s-file or empty
	// if (!strcmp(driver, K8S_FILE_STRING)) {
	// 	if (path == NULL) {
	// 		nexitf("k8s-file requires a filename");
	// 	}
	// 	use_k8s_logging = TRUE;
	// 	k8s_log_path = path;
	// 	return;
	// }

	// // If no : was found, use the entire log-path as a filename to k8s-file.
	// if (path == NULL && delim == NULL) {
	// 	use_k8s_logging = TRUE;
	// 	k8s_log_path = driver;
	// 	return;
	// }

	// nexitf("No such log driver %s", driver);

    
    Ok(())
}