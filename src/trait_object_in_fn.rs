/// 其中，impl Executor 使用的是泛型参数的简化版本，
/// 而 &dyn Executor 和 Box<dyn Executor> 是 trait object，前者在栈上，后者分配在堆上。
/// 值得注意的是，分配在堆上的 trait object 也可以作为返回值返回，比如示例中的 Result<Option<i32>, BoxedError> 里使用了 trait object。
use std::{error::Error, process::Command};

/// 这里为了简化代码，我使用了 type 关键字创建了一个 BoxedError 类型，是 Box<dyn Error + Send + Sync + 'static> 的别名，
/// 它是 Error trait 的 trait object，除了要求类型实现了 Error trait 外，它还有额外的约束：类型必须满足 Send / Sync 这两个 trait。
pub type BoxedError = Box<dyn Error + Send + Sync>;

pub trait Executor {
    fn run(&self) -> Result<Option<i32>, BoxedError>;
}

pub struct Shell<'a, 'b> {
    cmd: &'a str,
    args: &'b [&'a str],
}

impl <'a, 'b> Shell<'a, 'b> {
    pub fn new(cmd: &'a str, args: &'b [&'a str]) -> Self {
        Self { cmd, args }
    }
}

/// 运行Shell脚本, 返回结果
impl<'a, 'b> Executor for Shell<'a, 'b> {
    fn run(&self) -> Result<Option<i32>, BoxedError> {
       let output = Command::new(self.cmd).args(self.args).output()?;
        Ok(output.status.code())
    }
}

/// 使用泛型参数
pub fn execute_generics(cmd: &impl Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

/// 使用 trait object: &dyn T
pub fn execute_trait_object(cmd: &dyn Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

/// 使用 trait object: Box<dyn T>
pub fn execute_boxed_trait_object(cmd: Box<dyn Executor>) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_should_work() {
        let cmd = Shell::new("ls", &[]);
        let result = cmd.run().unwrap();
        assert_eq!(result, Some(0));
    }

    #[test]
    fn execute_should_work() {
        let cmd = Shell::new("ls", &[]);
        let result = execute_generics(&cmd).unwrap();
        assert_eq!(result, Some(0));

        let result = execute_trait_object(&cmd).unwrap();
        assert_eq!(result, Some(0));

        let boxed = Box::new(cmd);
        let result = execute_boxed_trait_object(boxed).unwrap();
        assert_eq!(result, Some(0));
    }
}