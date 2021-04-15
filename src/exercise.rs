use regex::Regex;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::fs::{remove_file, File};
use std::io::Read;
use std::process::{self, Command};
use std::path::PathBuf;

const I_AM_NOT_DONE_REGEX: &str = r"(?m)^\s*///?\s*I\s+AM\s+NOT\s+DONE";
const CONTEXT: usize = 2;

#[inline]
fn temp_file() -> String {
    let thread_id: String = format!("{:?}", std::thread::current().id())
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();
    format!("./temp_{}_{}", process::id(), thread_id)
}

#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

#[derive(Deserialize)]
pub struct Exercise {
    // 练习的名字
    pub name: String,
    // 练习源代码的文件路径
    pub path: PathBuf,
    // 练习的文字提示
    pub hint: String,
}

// 一个表示练习状态的枚举
#[derive(PartialEq, Debug)]
pub enum State {
    // 表示该练习已经通过编译
    Done,
    // 表示该练习还没有能够通过编译
    // 储存 包含 I AM NOT DONE 的行和这行的前后两行
    Pending(Vec<ContextLine>),
}

#[derive(PartialEq, Debug)]
pub struct ContextLine {
    // 储存一行内容
    pub line: String,
    // 储存行号 (从 1 开始)
    pub number: usize,
    // 是否是 I AM NOT DONE 那一行
    pub important: bool,
}

// 表示一个 exercise 的编译结果
pub struct CompiledExercise<'a> {
    exercise: &'a Exercise,
    _handle: FileHandle,
}

impl<'a> CompiledExercise<'a> {
    // 运行已经编译的 exercise
    pub fn run(&self) -> Result<ExerciseOutput, ExerciseOutput> {
        self.exercise.run()
    }
}

// 表示一个已经运行的二进制文件
#[derive(Debug)]
pub struct ExerciseOutput {
    // 这个二进制文件 standard output 输出的文本
    pub stdout: String,
    // 这个二进制文件 standard error 输出的文本
    pub stderr: String,
}

// 用于虚拟表示这一次练习(只是一个文件) exe 文件的句柄
// 当这一次练习(只是一个文件)结束后就会调用 drop 函数将这个临时exe删除
#[derive(Deserialize)]
struct FileHandle;

impl Drop for FileHandle {
    fn drop(&mut self) {
        clean();
    }
}

impl Exercise {
    // 将一个练习文件编译成 exe
    pub fn compile(&self) -> Result<CompiledExercise, ExerciseOutput> {
        // println!("while compile: {}", &temp_file());
        let cmd = Command::new("gcc")
            .args(&[self.path.to_str().unwrap(), "-o", &temp_file()])
            .output()
            .expect("Failed to run 'complie' command,");
        
            if cmd.status.success() {
                Ok(CompiledExercise {
                    exercise: &self,
                    _handle: FileHandle,
                })
            } else {
                clean();
                Err(ExerciseOutput {
                    stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
                })
            }
    }

    // 运行编译出的 exe 并返回运行结果
    fn run(&self) -> Result<ExerciseOutput, ExerciseOutput> {
        // 运行编译出的 exe 
        // println!("while run: {}", &temp_file());
        let cmd = Command::new(&temp_file())
            .output()
            .expect("Failed to run 'run' command");
        
        let output = ExerciseOutput {
            stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
            stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
        };

        // 返回编译结果
        if cmd.status.success() {
            Ok(output)
        } else {
            Err(output)
        }
    }

    // 获取一个练习所处的状态
    pub fn state(&self) -> State {
        let mut source_file = 
            File::open(&self.path).expect("Unable to open exercise file!");
        let source = {
            let mut s = String::new();
            source_file
                .read_to_string(&mut s)
                .expect("Unable to read the exercise file!");
                s
        };

        let re = Regex::new(I_AM_NOT_DONE_REGEX).unwrap();

        if !re.is_match(&source) {
            return State::Done;
        }

        // I AM NOT DONE 所在行的行号
        let matched_line_index = source
            .lines()
            .enumerate()
            .find_map(|(i, line)| if re.is_match(line) { Some(i) } else { None })
            .expect("This should not happen at all");
        
        // 最小行号 >= 0
        let min_line = ((matched_line_index as i32) - (CONTEXT as i32)).max(0) as usize;
        let max_line = matched_line_index + CONTEXT;

        let context = source
            .lines()
            .enumerate()
            .filter(|&(i, _)| i >= min_line && i <= max_line)
            .map(|(i, line)| ContextLine {
                line: line.to_string(),
                number: i + 1, // 将行号转为从 1 开始
                important: i == matched_line_index,
            })
            .collect();

        State::Pending(context)
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}

#[inline]
fn clean() {
    let _ignored = remove_file(&temp_file());
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    // 测试总结: 
    // 由于未知原因，当三个测试都执行时，run函数有近二分之一的失败概率
    // 但run函数测试单独运行时没有失败过

    // 测试 clean 函数是否正常运行
    #[test]
    fn test_clean() {
        File::create(&temp_file()).unwrap();
        let exercise = Exercise {
            name: String::from("example"),
            path: PathBuf::from("exercise_test/HelloWorld.c"),
            hint: String::from(""),
        };
        let compiled = exercise.compile().unwrap();
        drop(compiled);
        assert!(!Path::new(&temp_file()).exists());
    }

    // 测试 run 函数
    #[test]
    fn test_run() {
        let exercise = Exercise {
            name: String::from("example"),
            path: PathBuf::from("exercise_test/HelloWorldTWO.c"),
            hint: String::from(""),
        };
        let result = exercise.compile().unwrap().run().unwrap();
        assert!(result.stdout.contains("Hello World!"));
    }

    // 测试 state 函数
    #[test]
    fn state() {
        let exercise = Exercise {
            name: String::from("example"),
            path: PathBuf::from("exercise_test/HelloWorldThree.c"),
            hint: String::from(""),
        };

        let state = exercise.state();
        let expected = vec![
            ContextLine {
                line: "// exercise for test".to_string(),
                number: 2,
                important: false,
            },
            ContextLine {
                line: "".to_string(),
                number: 3,
                important: false,
            },
            ContextLine {
                line: "// I AM NOT DONE".to_string(),
                number: 4,
                important: true,
            },
            ContextLine {
                line: "".to_string(),
                number: 5,
                important: false,
            },
            ContextLine {
                line: "int main() {".to_string(),
                number: 6,
                important: false,
            },
        ];

        assert_eq!(state, State::Pending(expected));
    }
}