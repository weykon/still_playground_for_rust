use std::{future::Future, pin::Pin};

use tokio::io::AsyncRead;

use crate::File;

/// 源起
fn source_fn() {
    let mut buf = vec![0u8; 1024];

    // let content = some_file.read(&mut buf)?;

    // 以上的file跑的read是阻塞，
    // 直到buf这个可变引用不断被修改完成后才结束
    // 如果设计为非阻塞的Future的异步实现。
}

// 使用sleep来模拟，所以取名一下SlowRead
struct SlowRead<R> {
    reader: Pin<Box<R>>,
}
impl<R> SlowRead<R> {
    fn poll(reader: R) -> Self {
        Self {
            reader: Box::pin(reader),
        }
    }
}
impl<R> AsyncRead for SlowRead<R>
where
    R: AsyncRead,
{
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.reader.as_mut().poll_read(cx, buf)
    }
}
pub fn run() {}
