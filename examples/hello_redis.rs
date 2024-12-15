use mini_redis::{client, Result};

#[tokio::main]
async fn main()-> Result<()>  {
    // 建立与mini-redis服务器的连接
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 设置 key: "hello" 和 值: "world"
    let value = "world".into();
    client.set("hello", value).await?;
    client.set("myname", "xx".into()).await?;

    // 获取"key=hello"的值
    let result = client.get("hello").await?;
    let result2 = client.get("myname").await?;

    let value = match result {
        Some(value) => value,
        None => panic!("Something went wrong"),
    };

    println!("从服务器端获取到结果={:?}", value);
    println!("从服务器端获取到结果={:?}", result2);

    Ok(())
}
