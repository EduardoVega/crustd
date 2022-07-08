extern crate nix;

use clap::Parser;
use std::env;
use nix::unistd;
use anyhow::{Result,anyhow};

/// Container monitor for shimv2 runtime
#[derive(Parser)]
#[clap(author = "Eduardo Vega", version = "0.1.0", about, long_about = None)]
pub struct Cli {
    /// Conmon API version to use
    #[clap(long, required = false, required = false, default_value = "1")]
    pub api_version: u32,
    /// Location of the OCI Bundle path
    #[clap(parse(from_os_str))]
    #[clap(short = 'b', long, required = false, default_value = "")]
    pub bundle: std::path::PathBuf,
    /// Container ID
    #[clap(short = 'c', long)]
    pub cid: String,
    /// PID file for the conmon process
    #[clap(short = 'P', long, required = false, default_value = "")]
    pub conmon_pidfile: String,
    /// PID file for the initial pid inside of container
    #[clap(short = 'p', long, required = false, default_value = "")]
    pub container_pidfile: String,
    /// Container UUID
    #[clap(short = 'u', long, required = false, default_value = "")]
    pub cuuid: String,
    /// Exec a command into a running container
    #[clap(short = 'e', long)]
    pub exec: bool,
    /// Attach to an exec session
    #[clap(long)]
    pub exec_attach: bool,
    /// Path to the process spec for execution
    #[clap(long, required = false, default_value = "")]
    pub exec_process_spec: String,
    /// Path to the program to execute when the container terminates its execution
    #[clap(long, required = false, default_value = "")]
    pub exit_command: String,
    /// Additional arg to pass to the exit command.  Can be specified multiple times
    #[clap(long, required = false, default_value = "", multiple_occurrences = true)]
    pub exit_command_arg: Vec<String>,
    /// Delay before invoking the exit command (in seconds). Must be >= 0
    #[clap(long, required = false, default_value = "0")]
    pub exit_delay: u32,
    /// Path to the directory where exit files are written
    #[clap(long, required = false, default_value = "")]
    pub exit_dir: String,
    /// Leave stdin open when attached client disconnects
    #[clap(long)]
    pub leave_stdin_open: bool,
    /// Set logging verbosity based on log level
    #[clap(long, required = false, default_value = "error", possible_values = &["error", "warning", "info", "debug", "trace"])]
    pub log_level: String,
    /// Set logging verbosity based on occurrences of the `v/verbose` flag
    #[clap(short = 'v', long, parse(from_occurrences))]
    pub verbosity: usize,
    /// Silence all logging
    #[clap(short = 'q', long)]
    pub quiet: bool,
    /// Log file path
    #[clap(short = 'l', long, required = true, min_values = 1)]                  
    pub log_path: Vec<String>,
    /// Maximum size of log file
    #[clap(long, required = false, default_value = "-1")]
    pub log_size_max: i64,
    /// Additional tag to use for logging
    #[clap(long, required = false, default_value = "")]
    pub log_tag: String,
    /// Log to syslog (use with cgroupfs cgroup manager) - UNSUPPORTED
    #[clap(long)]
    pub syslog: bool,
    /// Container Name
    #[clap(short = 'n', long, required = false, default_value = "")]  
    pub name: String,
    /// Do not create a new session keyring for the container
    #[clap(long)]
    pub no_new_keyring: bool,
    /// Do not use pivot_root
    #[clap(long)]
    pub no_pivot: bool,
    /// Do not manually call sync on logs after container shutdown
    #[clap(long)]
    pub no_sync_log: bool,
    /// Persistent directory for a container that can be used for storing container data
    #[clap(short = '0', long, required = false, default_value = "")] 
    pub persist_dir: String,
    /// Replace listen pid if set for oci-runtime pid
    #[clap(long)]
    pub replace_listen_pid: bool,
    /// Restore a container from a checkpoint
    #[clap(long, required = false, default_value = "")]
    pub restore: String,
    /// Path to store runtime data for the container
     #[clap(short = 'r', long, required = true)]
    pub runtime: std::path::PathBuf,
    /// Additional arg to pass to the runtime. Can be specified multiple times
    #[clap(long, required = false, default_value = "", multiple_occurrences = true)]
    pub runtime_arg: Vec<String>, 
    /// Additional opts to pass to the restore or exec command. Can be specified multiple times
    #[clap(long, required = false, default_value = "")]
    pub runtime_opt: String,
    /// Path to the host's sd-notify socket to relay messages to
    #[clap(long, required = false, default_value = "")]
    pub sdnotify_socket: String, 
    /// Location of container attach sockets
    #[clap(long, required = false, default_value = "")]
    pub socket_dir_path: String,
    /// Open up a pipe to pass stdin to the container
    #[clap(short = 'i', long)]
    pub stdin: bool, 
    /// Keep the main conmon process as its child by only forking once
    #[clap(long)]
    pub sync: bool,
    /// Enable systemd cgroup manager, rather than use the cgroupfs directly
    #[clap(short = 's', long)] 
    pub systemd_cgroup: bool, 
    /// Allocate a pseudo-TTY. The default is false
    #[clap(short = 't', long)]
    pub terminal: bool,
    /// Kill container after specified timeout in seconds
    #[clap(short = 'T', long, required = false, default_value = "0")] 
    pub timeout: u32,
    /// Don't truncate the path to the attach socket. This option causes conmon to ignore --socket-dir-path
    #[clap(long)]
    pub full_attach: bool,           
    /// Path to the socket where the seccomp notification fd is received
    #[clap(long, required = false, default_value = "")] 
    pub seccomp_notify_socket: String,
    /// Plugins to use for managing the seccomp notifications
    #[clap(long, required = false, default_value = "")] 
    pub seccomp_notify_plugins: String,
}

impl Cli {
    pub fn process_cli(& mut self) -> Result<()>{
        // set logging verbosity
        let v = self.verbosity;

        // first we check --log-level
        if self.log_level == String::from("error"){
            self.verbosity = 0;
        } else if self.log_level == String::from("warning"){
            self.verbosity = 1;
        } else if self.log_level == String::from("info"){
            self.verbosity = 2;
        } else if self.log_level == String::from("debug"){
            self.verbosity = 3;
        } else if self.log_level == String::from("trace"){
            self.verbosity = 4;
        }

        // then we check -v, --verbosity, since
        // this will take precedence
        if v != 0 {
            self.verbosity = v;
        }

        // validations
        if self.restore != "" && self.exec {
            return Err(anyhow!("Cannot use 'exec' and 'restore' at the same time"));
        }

        if !self.exec && self.exec_attach {
            return Err(anyhow!("Attach can only be specified with exec"));
        }

        if self.api_version < 1 && self.exec_attach {
            return Err(anyhow!("Attach can only be specified for a non-legacy exec session"));
        }

        if self.cuuid == "" && (!self.exec || self.api_version >= 1){
            return Err(anyhow!("Container UUID not provided. Use --cuuid"));
        }

        if self.seccomp_notify_plugins == "" {
            match env::var("CONMON_SECCOMP_NOTIFY_PLUGINS"){
                Ok(v) => self.seccomp_notify_plugins = v,
                Err(_) => (),
            }
        }

        if self.runtime.exists(){
            return Err(anyhow!("Runtime path {:?} is not valid", self.runtime.to_str()))
        }

        if self.exec && self.exec_process_spec == "" {
            return Err(anyhow!("Exec process spec path not provided. Use --exec-process-spec"));
        }

        let cwd = unistd::getcwd()?;

        // opt_bundle_path in exec means we will set up the attach socket
        // for the exec session. the legacy version of exec does not need this
        // and thus we only override an empty opt_bundle_path when we're not exec
        if self.bundle.to_str() == Some("") && !self.exec {
            self.bundle = cwd;
        }

        // we should always override the container pid file if it's empty
        // TODO FIXME I removed default_pid_file here. shouldn't opt_container_pid_file be cleaned up?
        if self.container_pidfile == "" {
            self.container_pidfile = format!("{}/pidfile-{}", self.bundle.to_string_lossy(), self.cid);
        }

        Ok(())
    }
}