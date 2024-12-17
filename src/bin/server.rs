use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex,MutexGuard};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::yield_now;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
type ShardedDb =Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind(("0.0.0.0", 6379)).await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        //let db = db.clone();   Arc::clone(&db) == db.clone();
        let db = Arc::clone(&db);
        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

fn _test() {
    tokio::spawn(async {
        // 语句块的使用强制了 `rc` 会在 `.await` 被调用前就被释放，
        // 因此 `rc` 并不会影响 `.await`的安全性
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` 的作用范围已经失效，因此当任务让出所有权给当前线程时，它无需作为状态被保存起来
        yield_now().await;
    });
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};
    //use std::collections::HashMap;
    // let mut db = HashMap::new();

    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unsupported command: {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}


// fn _new_share_db(num_shards: usize) -> ShardedDb {
//     let mut db = Vec::with_capacity(num_shards);
//     for _ in 0..num_shards {
//         db.push(Mutex::new(HashMap::new()));
//     }
//     Arc::new(db)
// }



// async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
//     let mut lock: MutexGuard<i32> = mutex.lock().unwrap();
//     *lock += 1;
//     drop(lock);
//
//     do_something_async().await;
// }

