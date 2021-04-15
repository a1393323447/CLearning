use crate::exercise::{Exercise, ExerciseList};
use crate::run::run;
use crate::verify::verify;
use clap::{crate_version, App, Arg, SubCommand};
use console::Emoji;
use notify::DebouncedEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[macro_use]
mod ui;

mod exercise;
mod run;
mod verify;

fn main() {
    let matches = App::new("clearning")
        .version(crate_version!())
        .author("软院, 朕与将军解战袍")
        .about("clearning 是一个 c语言小练习的合集.希望通过 clearning ,你能够对 c语言的语法有初步的了解,并且养成良好的代码风格.")
        .arg(
            Arg::with_name("nocapture")
                .long("nocapture")
                .help("Show outputs from the test exercises")
        )
        .subcommand(
            SubCommand::with_name("verify")
                .alias("v")
                .about("Verifies all exercises according to the recommended order")
        )
        .subcommand(
            SubCommand::with_name("watch")
                .alias("w")
                .about("Reruns `verify` when files were edited")
        )
        .subcommand(
            SubCommand::with_name("run")
                .alias("r")
                .about("Runs/Tests a single exercise")
                .arg(Arg::with_name("name").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("hint")
                .alias("h")
                .about("Returns a hint for the current exercise")
                .arg(Arg::with_name("name").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("l")
                .about("Lists the exercises available in rustlings")
        )
        .get_matches();
    
    if matches.subcommand_name().is_none() {
        println!();
        println!(r#"                 welcome to ...                       "#);
        println!(r#"   _____   _                           _              "#);
        println!(r#"   / ____| | |                         (_)            "#);
        println!(r#"  | |      | |     ___  __ _ _ __ _ __  _ _ __   __ _ "#);
        println!(r#"  | |      | |    / _ \/ _` | '__| '_ \| | '_ \ / _` |"#);
        println!(r#"  | |____  | |___|  __/ (_| | |  | | | | | | | | (_| |"#);
        println!(r#"   \_____| |______\___|\__,_|_|  |_| |_|_|_| |_|\__, |"#);
        println!(r#"                                                 __/ |"#);
        println!(r#"                                                |___/ "#);
        println!();
    }

    if !Path::new("info.toml").exists() {
        println!(
            "请在 clearning 文件夹中打开 {}",
            std::env::current_exe().unwrap().to_str().unwrap()
        );
        println!("试着输入命令 `cd clearning/`!");
        std::process::exit(1);
    }

    if !gcc_exists() {
        println!("找不到 gcc 编译器.");
        println!("试着输入命令 `gcc --version` 看看有什么问题.");
        println!("百度一下: 怎么安装 gcc");
        std::process::exit(1);
    }

    let toml_str = &fs::read_to_string("info.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;
    let verbose = matches.is_present("nocapture");

    if matches.subcommand_matches("list").is_some() {
        exercises.iter().for_each(|e| println!("{}", e.name));
    }
    if let Some(ref matches) = matches.subcommand_matches("run") {
        let name = matches.value_of("name").unwrap();

        let matching_exercise = |e: &&Exercise| name == e.name;

        let exercise = exercises.iter().find(matching_exercise).unwrap_or_else(|| {
            println!("No exercise found for your given name!");
            std::process::exit(1)
        });

        run(&exercise).unwrap_or_else(|_| std::process::exit(1));
    }

    if let Some(ref matches) = matches.subcommand_matches("hint") {
        let name = matches.value_of("name").unwrap();

        let exercise = exercises
            .iter()
            .find(|e| name == e.name)
            .unwrap_or_else(|| {
                println!("没找到这个练习! 看看是不是输错名字了!");
                std::process::exit(1)
            });

        println!("{}", exercise.hint);
    }

    if matches.subcommand_matches("verify").is_some() {
        verify(&exercises).unwrap_or_else(|_| std::process::exit(1));
    }

    if matches.subcommand_matches("watch").is_some() {
        if let Err(e) = watch(&exercises, verbose) {
            println!("Error: 无法监视你的程序. 错误信息: {:?}.", e);
            println!("很可能是因为你磁盘内存满了 或者 你的 'inotify limit' 达到了上限.");
            std::process::exit(1);
        }
        println!(
            "{emoji} 恭喜你完成了所有的练习! {emoji}",
            emoji = Emoji("🎉", "★")
        );
        println!();
        println!("+--------------------------------------------------------+");     
        println!("|                    千里之行,始于足下                    |");       
        println!("+--------------------------------------------------------+");       
        println!("+＃＃＃＃＃＃＃＃＃＃＃＃＃ＬｆｆＬ＃＃＃＃＃＃＃＃＃＃＃＃＃+");
        println!("+＃＃＃＃＃＃＃＃＃＃＃＃ＬｆｆｆｆＬ＃＃＃＃＃＃＃＃＃＃＃＃+");
        println!("+＃＃＃＃＃＃＃＃＃＃ｆｆｆｆｆｆｆｆｆｆ＃＃＃＃＃＃＃＃＃＃+");
        println!("+＃＃＃＃＃＃＃＃ｆｆｆｆｆｆｆｆｆｆｆｆｆｆ＃＃＃＃＃＃＃＃+");
        println!("+＃＃＃＃＃＃ＫｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆＫ＃＃＃＃＃＃+");
        println!("+＃＃＃＃＃ｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆＬ＃＃＃＃＃+");
        println!("+＃＃＃Ｌｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆｆ＃＃＃+");
        println!("+＃Ｌｆｆｆｆｆｆｆｆｆｆ＃＃＃＃＃＃ｆｆｆｆｆｆｆｆｆｆＬ＃+");
        println!("+Ｌｆｆｆｆｆｆｆｆｆ＃＃＃＃＃＃＃＃＃＃ｆｆｆｆｆｆｆｆｆＬ+");
        println!("+ｆｆｆｆｆｆｆｆ＃＃＃＃＃＃＃＃＃＃＃＃＃＃ｆｆｆｆｆｆ；；+");
        println!("+ｆｆｆｆｆｆｆ＃＃＃＃＃＃＃＃＃＃＃＃＃＃＃＃ｆｆｆ；；；；+");
        println!("+ｆｆｆｆｆｆＬ＃＃＃＃＃＃＃＃＃＃＃＃＃＃＃＃Ｌｉ；；；；；+");
        println!("+ｆｆｆｆｆｆ＃＃＃＃＃＃＃ｆｆｆｆ＃＃＃＃＃Ｋ；；；；；；；+");
        println!("+ｆｆｆｆｆｆ＃＃＃＃＃ＤｆｆｆｆｆｆＤ＃＃；；；；；；；；；+");
        println!("+ｆｆｆｆｆ＃＃＃＃＃＃ｆｆｆｆｆｆｆｆ；；；；；；；；；；；+");
        println!("+ｆｆｆｆＬ＃＃＃＃＃ｆｆｆｆｆｆｆｆ；；；；；；；；；；；；+");
        println!("+ｆｆｆｆｆ＃＃＃＃＃ｆｆｆｆｆｆ；；；；；；；；；；；；；；+");
        println!("+ｆｆｆｆｆ＃＃＃＃＃ｆｆｆｆ，，；；；；；；；；；；；；；；+");
        println!("+ｆｆｆｆｆ＃＃＃＃＃ｆｆ，，，，，，；；；；；；；；；；；；+");
        println!("+ｆｆｆｆｆ＃＃＃＃＃＃，，，，，，，，；；；；；；；；；；；+");
        println!("+ｆｆｆｆｆｆ＃＃＃＃＃ｊ，，，，，，ｊ＃＃；；；；；；；；；+");
        println!("+ｆｆｆｆｆｆ＃＃＃＃＃＃＃，，，，＃＃＃＃＃Ｋ；；；；；；；+");
        println!("+ｆｆｆｆｆｆｉ＃＃＃＃＃＃＃＃＃＃＃＃＃＃＃＃ｉ；；；；；；+");
        println!("+ｆｆｆＬ，，，＃＃＃＃＃＃＃＃＃＃＃＃＃＃＃＃，，，；；；；+");
        println!("+ｆｆ，，，，，，＃＃＃＃＃＃＃＃＃＃＃＃＃＃，，，，，，；；+");
        println!("+，，，，，，，，，：＃＃＃＃＃＃＃＃＃＃：，，，，，，，，，+");
        println!("+＃；，，，，，，，，，，＃＃＃＃＃＃：，，，，，，，，，；＃+");
        println!("+＃＃＃，，，，，，，，，，，，，，，，，，，，，，，，＃＃＃+");
        println!("+＃＃＃＃＃，，，，，，，，，，，，，，，，，，，，＃＃＃＃＃+");
        println!("+＃＃＃＃＃＃Ｄ，，，，，，，，，，，，，，，，Ｄ＃＃＃＃＃＃+");
        println!("+＃＃＃＃＃＃＃＃，，，，，，，，，，，，，，＃＃＃＃＃＃＃＃+");
        println!("+＃＃＃＃＃＃＃＃＃＃，，，，，，，，，，＃＃＃＃＃＃＃＃＃＃+");
        println!("+＃＃＃＃＃＃＃＃＃＃＃＃，，，，，，＃＃＃＃＃＃＃＃＃＃＃＃+");
        println!("+＃＃＃＃＃＃＃＃＃＃＃＃＃；，，；＃＃＃＃＃＃＃＃＃＃＃＃＃+");
        println!();
        println!("希望通过 clearining 的练习，你对 C语言有了初步的了解.");
        println!("如果想要更加深入地了解 C语言 或者 C++, 你可以复制下方链接, 到浏览器进行浏览:");
        println!("https://zh.cppreference.com/");
        println!();
    }

    if matches.subcommand_name().is_none() {
        let text = fs::read_to_string("default_out.txt").unwrap();
        println!("{}", text);
    }

}

fn spawn_watch_shell(failed_exercise_hint: &Arc<Mutex<Option<String>>>) {
    let failed_exercise_hint = Arc::clone(failed_exercise_hint);
    println!("输入 'hint' 查看提示 或者 输入 'clear' 清屏");
    thread::spawn(move || loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input.eq("hint") {
                    if let Some(hint) = &*failed_exercise_hint.lock().unwrap() {
                        println!("{}", hint);
                    }
                } else if input.eq("clear") {
                    println!("\x1B[2J\x1B[1;1H");
                } else {
                    println!("unknown command: {}", input);
                }
            }
            Err(error) => println!("error reading command: {}", error),
        }
    });
}

fn watch(exercises: &[Exercise], verbose: bool) -> notify::Result<()> {
    /* Clears the terminal with an ANSI escape code.
    Works in UNIX and newer Windows terminals. */
    fn clear_screen() {
        println!("\x1Bc");
    }

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
    watcher.watch(Path::new("./exercises"), RecursiveMode::Recursive)?;

    clear_screen();

    let to_owned_hint = |t: &Exercise| t.hint.to_owned();
    let failed_exercise_hint = match verify(exercises.iter()) {
        Ok(_) => return Ok(()),
        Err(exercise) => Arc::new(Mutex::new(Some(to_owned_hint(exercise)))),
    };
    spawn_watch_shell(&failed_exercise_hint);
    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(b) | DebouncedEvent::Chmod(b) | DebouncedEvent::Write(b) => {
                    if b.extension() == Some(OsStr::new("c")) && b.exists() {
                        let filepath = b.as_path().canonicalize().unwrap();
                        let pending_exercises = exercises
                            .iter()
                            .skip_while(|e| !filepath.ends_with(&e.path));
                        clear_screen();
                        match verify(pending_exercises) {
                            Ok(_) => return Ok(()),
                            Err(exercise) => {
                                let mut failed_exercise_hint = failed_exercise_hint.lock().unwrap();
                                *failed_exercise_hint = Some(to_owned_hint(exercise));
                            }
                        }
                    }
                }
                _ => {}
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn gcc_exists() -> bool {
    Command::new("gcc")
        .args(&["--version"])
        .stdout(Stdio::null())
        .spawn()
        .and_then(|mut child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}
