use std::env;

fn main() {
    let mut args = env::args();

    let bin = args.next().unwrap();

    if args.len() < 1 {
        println!("Provide a command");
        return;
    }


    match args.next().unwrap().as_str() {
        "compile" => {
            if args.len() < 2 {
                println!("Usage: {bin} compile <source.rasm> <output.rvim>");
                return;
            }

            let src_path = args.next().unwrap();
            let mut src = match std::fs::File::open(&src_path) {
                Ok(src) => src,
                Err(why) => {
                    eprintln!("Error opening source file {}: {}", src_path, why);
                    return;
                }
            };
            
            let out_path = args.next().unwrap();
            let mut out = match std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(&out_path) 
            {
                Ok(out) => out,
                Err(why) => {
                    eprintln!("Error opening output file {}: {}", out_path, why);
                    return;
                }
            };

            if let Err(why) = rvim::compile(&mut src, &mut out) {
                eprintln!("Error compiling source: {:?}", why);
            }
        },
        "run" => {
            if args.len() < 1 {
                println!("Usage: {bin} run <program.rvim>");
                return;
            }

            let path = args.next().unwrap();

            if let Ok(mut file) = std::fs::File::open(&path) {
                match rvim::run_file(&mut file) {
                    Err(why) => {
                        eprintln!("Error executing program: {:?}", why);
                        return;
                    },
                    Ok(vm) => vm.stack_dump(),
                }
            } else {
                eprintln!("Could not read file \"{}\"", path);
            }
        }
        _ => {
            println!("Bad command");
        }
    }
}
