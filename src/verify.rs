use crate::exercise::{CompiledExercise, Exercise, State};
use console::style;
use indicatif::ProgressBar;

pub fn verify<'a>(
    start_at: impl IntoIterator<Item = &'a Exercise>,
) -> Result<(), &'a Exercise> {
    for exercise in start_at {
        let compile_result = compile_and_run_interactively(&exercise);
        if !compile_result.unwrap_or(false) {
            return Err(exercise);
        }
    }
    Ok(())
}

fn compile_and_run_interactively(exercise: &Exercise) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("正在编译 {}...", exercise).as_str());
    progress_bar.enable_steady_tick(100);

    let compilation = compile(&exercise, &progress_bar)?;

    progress_bar.set_message(format!("正在运行 {}...", exercise).as_str());
    let result = compilation.run();
    progress_bar.finish_and_clear();

    let output = match result {
        Ok(output) => output,
        Err(output) => {
            warn!("{} 有如下错误", exercise);
            println!("{}", output.stdout);
            println!("{}", output.stderr);
            return Err(());
        }
    };

    success!("{} 成功运行!", exercise);

    Ok(prompt_for_completion(&exercise, Some(output.stdout)))
}

fn compile<'a, 'b>(
    exercise: &'a Exercise,
    progress_bar: &'b ProgressBar,
) -> Result<CompiledExercise<'a>, ()> {
    let compilation_result = exercise.compile();

    match compilation_result {
        Ok(compilation) => Ok(compilation),
        Err(output) => {
            progress_bar.finish_and_clear();
            warn!(
                "{} 编译失败! 再试一次. 这一次的输出如下:",
                exercise
            );
            println!("{}", output.stderr);
            Err(())
        }
    }
}

fn prompt_for_completion(exercise: &Exercise, prompt_output: Option<String>) -> bool {
    let context = match exercise.state() {
        State::Done => return true,
        State::Pending(context) => context,
    };

    println!();
    println!("🎉 🎉  通过编译了! 🎉 🎉");
    println!();

    if let Some(output) = prompt_output {
        println!("输出:");
        println!("{}", separator());
        println!("{}", output);
        println!("{}", separator());
        println!();
    }

    println!("你可以继续改一下这一次练习的代码,");
    println!(
        "或者去掉 {} 这个注释，然后做下一个练习:",
        style("`I AM NOT DONE`").bold()
    );
    println!();
    for context_line in context {
        let formatted_line = if context_line.important {
            format!("{}", style(context_line.line).bold())
        } else {
            context_line.line.to_string()
        };

        println!(
            "{:>2} {}  {}",
            style(context_line.number).blue().bold(),
            style("|").blue(),
            formatted_line
        );
    }

    false
}

fn separator() -> console::StyledObject<&'static str> {
    style("====================").bold()
}