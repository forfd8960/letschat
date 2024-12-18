use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = broadcast::channel(10);
    let mut rx2 = tx.subscribe();

    let handle1 = tokio::spawn(async move {
        println!("received: {} in feature1", rx.recv().await.unwrap());
    });

    let handle2 = tokio::spawn(async move {
        println!("received: {} in feature2", rx2.recv().await.unwrap());
    });

    tx.send(100).unwrap();

    let _ = handle1.await;
    let _ = handle2.await;
}
