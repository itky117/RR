use RR::compiler::{compile, CliLog, OptLevel};
use RR::runtime::runner::Runner;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    let code = match args[1].as_str() {
        "build" => cmd_build(&args[2..]),
        "run" => cmd_run(&args[2..]),
        _ => cmd_legacy(&args[1..]),
    };

    if code != 0 {
        std::process::exit(code);
    }
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  rr <input.rr> [options]");
    eprintln!("  rr run [main.rr|dir|.] [options]");
    eprintln!("  rr build [dir|file.rr] [options]");
    eprintln!("Options:");
    eprintln!("  -o <file> / --out-dir <dir>   Output file (legacy) or build output dir");
    eprintln!("  -O0, -O1, -O2                 Optimization level (default O1)");
    eprintln!("  -o0, -o1, -o2                 (Also accepted) Optimization level");
    eprintln!("  --keep-r                      Keep generated .gen.R when running");
    eprintln!("  --no-runtime                  Compile only (legacy mode)");
}

fn apply_opt_flag(arg: &str, level: &mut OptLevel) -> bool {
    if arg == "-O0" || arg == "-o0" {
        *level = OptLevel::O0;
        true
    } else if arg == "-O1" || arg == "-o1" {
        *level = OptLevel::O1;
        true
    } else if arg == "-O2" || arg == "-O" || arg == "-o2" {
        *level = OptLevel::O2;
        true
    } else {
        false
    }
}

fn cmd_legacy(args: &[String]) -> i32 {
    let ui = CliLog::new();
    let mut input_path = String::new();
    let mut output_path = None;
    let mut keep_r = false;
    let mut opt_level = OptLevel::O1;
    let mut no_runtime = false;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-o" {
            if i + 1 < args.len() {
                output_path = Some(args[i + 1].clone());
                i += 1;
            } else {
                ui.error("Missing output file after -o");
                return 1;
            }
        } else if apply_opt_flag(arg, &mut opt_level) {
            // handled
        } else if arg == "--keep-r" {
            keep_r = true;
        } else if arg == "--no-runtime" {
            no_runtime = true;
        } else if arg == "--mir" {
            if matches!(opt_level, OptLevel::O0) {
                opt_level = OptLevel::O1;
            }
        } else if !arg.starts_with('-') {
            input_path = arg.clone();
        }
        i += 1;
    }

    if input_path.is_empty() {
        print_usage();
        return 1;
    }
    if !input_path.ends_with(".rr") {
        ui.error("Input file must end with .rr");
        return 1;
    }

    let input = match fs::read_to_string(&input_path) {
        Ok(s) => s,
        Err(e) => {
            ui.error(&format!(
                "Failed to read input file '{}': {}",
                input_path, e
            ));
            return 1;
        }
    };

    let result = compile(&input_path, &input, opt_level);
    match result {
        Ok((r_code, source_map)) => {
            if let Some(out_path) = output_path {
                if let Err(e) = fs::write(&out_path, &r_code) {
                    ui.error(&format!(
                        "Failed to write output file '{}': {}",
                        out_path, e
                    ));
                    return 1;
                }
                ui.success(&format!("Compiled to {}", out_path));
                0
            } else if !no_runtime {
                Runner::run(&input_path, &input, &r_code, &source_map, None, keep_r)
            } else {
                ui.success("Compilation successful (runtime skipped)");
                0
            }
        }
        Err(e) => {
            e.display(Some(&input), Some(&input_path));
            1
        }
    }
}

fn resolve_run_input(raw: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(raw);
    if path.is_dir() || raw == "." {
        let entry = path.join("main.rr");
        if entry.is_file() {
            Ok(entry)
        } else {
            Err(format!("main.rr not found in '{}'", path.to_string_lossy()))
        }
    } else if path.is_file() {
        if path.extension().and_then(|s| s.to_str()) == Some("rr") {
            Ok(path)
        } else {
            Err("run target must be a .rr file or directory".to_string())
        }
    } else {
        Err(format!("run target not found: '{}'", raw))
    }
}

fn cmd_run(args: &[String]) -> i32 {
    let ui = CliLog::new();
    let mut target = ".".to_string();
    let mut keep_r = false;
    let mut opt_level = OptLevel::O1;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if apply_opt_flag(arg, &mut opt_level) {
            // handled
        } else if arg == "--keep-r" {
            keep_r = true;
        } else if !arg.starts_with('-') {
            target = arg.clone();
        }
        i += 1;
    }

    let input_path = match resolve_run_input(&target) {
        Ok(p) => p,
        Err(msg) => {
            ui.error(&msg);
            return 1;
        }
    };
    let input_path_str = input_path.to_string_lossy().to_string();
    let input = match fs::read_to_string(&input_path) {
        Ok(s) => s,
        Err(e) => {
            ui.error(&format!("Failed to read '{}': {}", input_path_str, e));
            return 1;
        }
    };

    match compile(&input_path_str, &input, opt_level) {
        Ok((r_code, source_map)) => {
            Runner::run(&input_path_str, &input, &r_code, &source_map, None, keep_r)
        }
        Err(e) => {
            e.display(Some(&input), Some(&input_path_str));
            1
        }
    }
}

fn collect_rr_files(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if name == "build" || name == "target" || name == ".git" {
                continue;
            }
            collect_rr_files(&path, files)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("rr") {
            files.push(path);
        }
    }
    Ok(())
}

fn cmd_build(args: &[String]) -> i32 {
    let ui = CliLog::new();
    let mut target = ".".to_string();
    let mut out_dir = "build".to_string();
    let mut opt_level = OptLevel::O1;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--out-dir" || arg == "-o" {
            if i + 1 < args.len() {
                out_dir = args[i + 1].clone();
                i += 1;
            } else {
                ui.error(&format!("Missing directory path after {}", arg));
                return 1;
            }
        } else if apply_opt_flag(arg, &mut opt_level) {
            // handled
        } else if !arg.starts_with('-') {
            target = arg.clone();
        }
        i += 1;
    }

    let target_path = PathBuf::from(&target);
    if !target_path.exists() {
        ui.error(&format!("build target not found: '{}'", target));
        return 1;
    }

    let out_root = PathBuf::from(&out_dir);
    if let Err(e) = fs::create_dir_all(&out_root) {
        ui.error(&format!(
            "Failed to create output directory '{}': {}",
            out_dir, e
        ));
        return 1;
    }
    println!("{} {}", ui.yellow_bold("[+]"), ui.red_bold("RR Build"));
    println!(
        " {} {}",
        ui.dim("└─"),
        ui.white_bold(&format!(
            "Target: {} | Out: {} ({})",
            target,
            out_dir,
            opt_level.label()
        ))
    );

    let mut rr_files = Vec::new();
    let dir_mode = target_path.is_dir();
    if dir_mode {
        if let Err(e) = collect_rr_files(&target_path, &mut rr_files) {
            ui.error(&format!("Failed while scanning '{}': {}", target, e));
            return 1;
        }
    } else if target_path.extension().and_then(|s| s.to_str()) == Some("rr") {
        rr_files.push(target_path.clone());
    } else {
        ui.error("build target must be a directory or .rr file");
        return 1;
    }

    rr_files.sort();
    if rr_files.is_empty() {
        ui.error(&format!("no .rr files found under '{}'", target));
        return 1;
    }

    let root_abs = if dir_mode {
        fs::canonicalize(&target_path).unwrap_or(target_path.clone())
    } else {
        PathBuf::new()
    };

    let mut built = 0usize;
    for rr in rr_files {
        let rr_abs = fs::canonicalize(&rr).unwrap_or(rr.clone());
        let rr_path_str = rr_abs.to_string_lossy().to_string();
        let input = match fs::read_to_string(&rr_abs) {
            Ok(s) => s,
            Err(e) => {
                ui.error(&format!("Failed to read '{}': {}", rr_path_str, e));
                return 1;
            }
        };

        let (r_code, _source_map) = match compile(&rr_path_str, &input, opt_level) {
            Ok(v) => v,
            Err(e) => {
                e.display(Some(&input), Some(&rr_path_str));
                return 1;
            }
        };

        let out_file = if dir_mode {
            let rel = rr_abs.strip_prefix(&root_abs).unwrap_or(&rr_abs);
            out_root.join(rel).with_extension("R")
        } else {
            let stem = rr.file_stem().and_then(|s| s.to_str()).unwrap_or("out");
            out_root.join(format!("{}.R", stem))
        };

        if let Some(parent) = out_file.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                ui.error(&format!("Failed to create '{}': {}", parent.display(), e));
                return 1;
            }
        }
        if let Err(e) = fs::write(&out_file, r_code) {
            ui.error(&format!("Failed to write '{}': {}", out_file.display(), e));
            return 1;
        }

        ui.success(&format!("Built {} -> {}", rr.display(), out_file.display()));
        built += 1;
    }

    ui.success(&format!(
        "Build complete: {} file(s) -> {}",
        built,
        out_root.display()
    ));
    0
}
