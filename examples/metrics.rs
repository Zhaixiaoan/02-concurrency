use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::{thread, time::Duration};

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrice = Metrics::new();
    println!("{}", metrice);

    for id in 0..N {
        task_work(id, metrice.clone())?;
    }

    for _ in 0..M {
        request_work(metrice.clone())?;
    }
    println!("task done");
    loop {
        thread::sleep(Duration::from_secs(1));
        println!("{}", metrice);
    }

    #[allow(unreachable_code)]
    //常见task_work
    Ok(())
}

fn task_work(id: usize, metrice: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrice.inc(format!("task_{}", id))?;
        }

        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_work(metrice: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..500)));
            let page = rng.gen_range(1..5);
            metrice.inc(format!("task_page:{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
