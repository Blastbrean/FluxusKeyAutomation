use std::io::{self, Read, Write};

fn pause() {
    let mut stdout = io::stdout();
    stdout.write(b"press enter to continue...").unwrap();
    stdout.flush().unwrap();
    io::stdin().read(&mut [0]).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut buffer = String::new();
    let client = reqwest::Client::new();

    // ask user hwid...
    println!("welcome to the fluxus key automation tool!");
    println!("what is your hwid?");
    println!("you can obtain this through pressing the \"get key\" button on fluxus's launcher.");

    // attempt to read line...
    match io::stdin().read_line(&mut buffer) {
        Ok(_line) => {}
        Err(err) => println!("failed to get read-line: {}", err),
    }

    // parse response...
    let response = match buffer.trim_end() {
        "" => panic!("failed to obtain hwid..."),
        hwid => format!("automating with hwid: {hwid}"),
    };

    // ok, we start from one...
    let params = [("updated_browser", "true"), ("HWID", &response)];
    client
        .post("https://flux.li/windows/start.php?updated_browser=true")
        .form(&params)
        .send()
        .await?;

    // check 1...
    let res_check_one = client.post("https://fluxteam.net/windows/checkpoint/check1.php")
           .form(&params)
           .header("Referer", "https://linkvertise.com/")
           .header("Sec-Fetch-Site", "cross-site")
           .header("Sec-Fetch-Mode", "navigate")
           .header("Sec-Fetch-Dest", "document")
           .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
           .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36")
           .send()
           .await?
           .text()
           .await?;

    if res_check_one
        == "Trying to bypass the Fluxus key system will get you banned from using Fluxus."
    {
        println!("failed to bypass res_check_two...");
        pause();
    } else {
        println!("bypassed res_check_one..");
    }

    // check 2...
    let res_check_two = client.post("https://fluxteam.net/windows/checkpoint/check2.php")
           .form(&params)
           .header("Referer", "https://linkvertise.com/")
           .header("Sec-Fetch-Site", "cross-site")
           .header("Sec-Fetch-Mode", "navigate")
           .header("Sec-Fetch-Dest", "document")
           .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
           .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36")
           .send()
           .await?
           .text()
           .await?;

    if res_check_two
        == "Trying to bypass the Fluxus key system will get you banned from using Fluxus."
    {
        println!("failed to bypass res_check_two...");
        pause();
    } else {
        println!("bypassed res_check_two..");
    }

    // check final...
    let res_check_final = client.post("https://fluxteam.net/windows/checkpoint/main.php")
           .form(&params)
           .header("Referer", "https://linkvertise.com/")
           .header("Sec-Fetch-Site", "cross-site")
           .header("Sec-Fetch-Mode", "navigate")
           .header("Sec-Fetch-Dest", "document")
           .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7")
           .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36")
           .send()
           .await?
           .text()
           .await?;

    if res_check_final
        == "Trying to bypass the Fluxus key system will get you banned from using Fluxus."
    {
        println!("failed to bypass res_check_final...");
        pause();
    } else {
        println!("bypassed res_check_final..");
    }

    // print out response...
    println!("finished automation!");

    // scan lines for our key...
    for line in res_check_final.lines() {
        // find "let content" (contains the key...)
        let key_in_this_line = line.contains("let content");
        if !key_in_this_line {
            continue;
        }

        // print key...
        println!(
            "here is your key: {}",
            line.replace("let content = (\"", "")
                .replace("\");", "")
                .trim()
        );
        pause();

        // break out of loop by returning...
        return Ok(());
    }

    // failed to find key...
    println!(
        "failed to find key, report this output to blastbrean on discord: {} -> {}",
        res_check_one, res_check_final
    );
    pause();

    // finish execution...
    Ok(())
}
