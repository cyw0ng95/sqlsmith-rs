use sqlsmith_rs_common::profile::Profile;
use std::env;
use std::path::Path;
use std::process::exit;
use tokio::process::Command;

fn get_executor_path() -> Option<String> {
    env::current_exe().ok().and_then(|mut path| {
        path.pop();
        path.push("executor");
        path.to_str().map(|s| s.to_string())
    })
}

fn can_execute(path: &str) -> bool {
    Path::new(path).exists() && std::process::Command::new(path).arg("--version").output().is_ok()
}

/// fork_server 的主函数，用于生成多个进程
pub async fn fork_server_main(profile: &Profile) {
    let executor_count = profile.executor_count.unwrap();
    let base_seed = profile.seed.unwrap_or(0);
    println!("Using executor count: {}", executor_count);

    let executor_path = match get_executor_path() {
        Some(path) if can_execute(&path) => path,
        _ => {
            eprintln!("Cannot find or execute the executor binary");
            exit(1);
        }
    };

    let mut handles = Vec::new();
    for n in 0..executor_count {
        let path = executor_path.clone();
        let process_name = format!("exec_{}", n);
        let seed = (base_seed << 8) + n as u64;
        let handle = tokio::spawn(async move {
            let mut cmd = Command::new(&path);
            cmd.env("EXEC_PARAM_SEED", seed.to_string());
            
            #[cfg(unix)]
            {
                use std::os::unix::process::CommandExt;
                cmd.arg0(&process_name);
                
                // Set up prctl to kill child when parent dies
                unsafe {
                    cmd.pre_exec(|| {
                        // PR_SET_PDEATHSIG = 1
                        // SIGTERM = 15
                        let ret = libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM);
                        if ret != 0 {
                            eprintln!("Failed to set PR_SET_PDEATHSIG");
                        }
                        Ok(())
                    });
                }
            }
            
            match cmd.spawn() {
                Ok(mut child) => {
                    if let Err(e) = child.wait().await {
                        eprintln!("Executor failed: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to execute {}: {}", path, e);
                    exit(1);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.await;
    }
}
