use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(10);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        println!("received: {} in feature1", rx.recv().await.unwrap());
    });

    tokio::spawn(async move {
        println!("received: {} feature2", rx2.recv().await.unwrap());
    });
    tx.send(100).unwrap();
}
