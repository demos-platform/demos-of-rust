use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::sync::mpsc;

enum Message {
  NewJob(Job),
  Terminate,
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
  /// 创建线程池
  ///
  /// 线程池中线程的数量。
  ///
  /// # Panics
  ///
  /// `new` 函数会在 size 为 0 时触发 panic。
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      // 创建线程并将它们存储至动态数组中
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool {
      workers,
      sender,
    }
  }

  pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static {
      let job = Box::new(f);
      self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Sending terminate message to all workers.");

    // 标记 ①。
    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    println!("Shutting down all workers.");

    // 标记 ②。
    // 标记 ①、② 使用两个循环而未合并用一个循环的目的是为了避免出现死锁问题。
    // 这么做可以避免死锁问题的原因为在 ① 中对所有 work 发送结束信号，所有 work 都后会停止接受请求。
    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);

      // 目的：得到所有权。为 Option 值调用 take 方法会将 Some 变体的值移出并在原有位置留下 None 变体。
      if let Some(thread) = worker.thread.take() {
        // Todo：确认当前代码貌似无效
        // 目的：使正在处理的工作线程不会因为一些异常情况（如 ctrl + c 停止主线程）终止运行。
        thread.join().unwrap();
      }
    }
  }
}

struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || {
      loop {
        let message: Message = receiver.lock().unwrap().recv().unwrap();

        match message {
          Message::NewJob(job) => {
            println!("Worker {} got a job; executing.", id);

            job();
          },
          Message::Terminate => {
            println!("Worker {} was told to terminate.", id);

            break;
          },
        }
      }
    });

    Worker {
      id,
      thread: Some(thread),
    }
  }
}
