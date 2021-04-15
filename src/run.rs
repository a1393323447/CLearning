use crate::exercise::Exercise;
use indicatif::ProgressBar;

pub fn run(exercise: &Exercise) -> Result<(), ()> {
    compile_and_run(exercise)?;
    Ok(())
}


fn compile_and_run(exercise: &Exercise) -> Result<(), ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {}...", exercise).as_str());
    progress_bar.enable_steady_tick(100);

    let compilation_result = exercise.compile();
    let compilation = match compilation_result {
        Ok(compilation) => compilation,
        Err(output) => {
            progress_bar.finish_and_clear();
            warn!(
                "{} 编译失败, 错误信息如下:\n",
                exercise
            );
            println!("{}", output.stderr);
            return Err(());
        }
    };

    progress_bar.set_message(format!("正在运行 {}...", exercise).as_str());
    let result = compilation.run();
    progress_bar.finish_and_clear();

    match result {
        Ok(output) => {
            println!("{}", output.stdout);
            success!("成功运行 {}", exercise);
            Ok(())
        },
        Err(output) => {
            println!("{}", output.stdout);
            println!("{}", output.stderr);

            warn!("{} 有如下错误", exercise);
            Err(())
        }
    }
}