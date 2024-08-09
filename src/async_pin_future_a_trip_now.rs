use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{thread::sleep, time::Duration};
use tokio::runtime::{Builder, Runtime};
use tokio::time::{sleep as tokio_sleep, Sleep};

pub fn run() {
    println!("async_pin_future_a_trip_now");

    // 默认下基于对多线程的支持
    let default_multiple_rt = Runtime::new().unwrap();
    default_multiple_rt.block_on(async {
        two_with_join().await;
    });

    // 设置为在当前线程上运行
    println!("---");
    let one_thread_rt = Builder::new_current_thread().enable_all().build().unwrap();
    one_thread_rt.block_on(async {
        // 来源std的sleep仅仅调用系统的sleep，阻塞当前线程。
        two_with_join().await;
        // tokio的sleep是异步的，不会阻塞当前线程。
        // 文章里说到这歌sleep返回一个Future，在第一次轮询它时，它会注册一个计时器，
        // 直到超时了，它才会完成。
        two_with_join_use_tokio_sleep().await;

        future_course().await;
    });
}

async fn greet() {
    println!("Hello!");
    sleep(Duration::from_millis(500));
    println!("(500ms passed...)");
    println!("Goodbye!");
}

async fn two_with_join() {
    let one = tokio::spawn(greet());
    let two = tokio::spawn(greet());
    let (_, _) = tokio::join!(one, two);
}

async fn use_tokio_sleep() {
    tokio_sleep(Duration::from_millis(500)).await;
    println!("(tokio sleep - 500ms passed...)");
}

async fn greet_with_tokio_sleep() {
    println!("Hello!");
    use_tokio_sleep().await;
    println!("Goodbye!");
}

// 这个的sleep，不阻塞。 而std当中的会阻塞。
async fn two_with_join_use_tokio_sleep() {
    println!("---two_with_join_use_tokio_sleep");
    let one = tokio::spawn(greet_with_tokio_sleep());
    let two = tokio::spawn(greet_with_tokio_sleep());
    let (_, _) = tokio::join!(one, two);
}

// Future is just a trait.
struct FarFuture {
    // me_I future
    slept: bool,
}
impl Future for FarFuture {
    type Output = ();

    // 在这个poll，轮询里面的code，
    // 是以对这个future的启发机制的整个过程的code
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // todo!()
        println!("pending ... ");
        // cx.waker().wake_by_ref();
        // let weak = cx.waker().clone();
        // Weak
        /// Implements [`Clone`], [`Send`], and [`Sync`]; therefore, a waker may be invoked
        /// from any thread, including ones not in any way managed by the executor. For example,
        /// this might be done to wake a future when a blocking function call completes on another
        /// thread.
        // 一秒后被唤醒 （新开一个线程每隔一秒去唤醒这个轮询）
        // 目前这样每隔一秒的唤醒，而返回Pending，是保持事件继续等待而可以进行下去
        // 线程中的wake()是再次执行poll函数，所以每次poll都创建了一个线程然后重新用
        // std的sleep计时，又pending下去。
        // （当然，不断开创线程，是不讲究性能了的）
        //1 std::thread::spawn(move || {
        //1     std::thread::sleep(Duration::from_secs(1));
        //1     weak.wake();
        //1 });
        // 下方新代码
        // 添加一个变量给self，在参考这个self可能在外界被修改后
        // 从此处检查去返回ready
        match self.slept {
            false => {
                let waker = cx.waker().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_secs(1));
                    waker.wake();
                });
                // 这里上下语句是异步的，这里是先执行了为true，然后在下一秒去wake(),poll来检查
                self.slept = true;
                Poll::Pending
            }
            true => Poll::Ready(()),
        }
    }
}

async fn future_course() {
    // 以下两行执行
    let fut = FarFuture { slept: false };
    println!("future_course: Start wait");
    fut.await;
    println!("future_course: Done!");
    // 以上两行
    // 会有 not yet implemented 的报错
    // 在这个fut中原本todo!()的代码，没有返回poll，就报错。

    println!("future_course: Start wait TokioTimeFuture");
    let tokio_time_future = TokioTimeFuture {
        sleep: Box::pin(tokio_sleep(Duration::from_secs(1))),
    };
    tokio_time_future.await;
    println!("future_course: Done! TokioTimeFuture");
}

struct TokioTimeFuture {
    sleep: Pin<Box<Sleep>>,
}
impl Future for TokioTimeFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("TokioTimeFuture: pending ... ");
        self.sleep.as_mut().poll(cx)
    }
}
