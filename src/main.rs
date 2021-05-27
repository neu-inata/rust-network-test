use std::net::TcpListener;
use std::io::Read;
use std::str;


fn main() -> std::io::Result<()> {
    // リスナーの作成
    let listener = TcpListener::bind("127.0.0.1:7777")?;

    // 接続待ち
    let (mut stream, _) = listener.accept()?;

    loop {
        let mut buf = [0u8; 1024];
        let res = stream.read(&mut buf);
        match res {
            Result::Ok(sz) => {
                // 切断確認
                if sz == 0{
                    println!("close!");
                    break;
                }
                println!("sz={} buf:{}", sz, str::from_utf8(&buf[..sz]).unwrap());
            },
            Result::Err(err) => {
                println!("{}", err.to_string());
                break;
            }
        }
    }
    Ok(())
}
