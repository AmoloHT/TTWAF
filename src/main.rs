use std::error::Error;
use std::io::{self, BufRead};
use std::time::Duration;
use std::env;

fn banner() {
    println!("\x1b[95m
                 XOOkxdddddxxkOKXN                 
             NKkdo:loooooooooooo:lkK               
           Nkooooo:loooooooooooo::oookX            
         Nklc;ccccccccccccc:cccccccccclO           
        Ndooo;loooooooooooo:oooooooooooodX         
       Xdoooo;lod0KXXNN    NNXK0OdooooooocX        
       occccc:cc0       \x1b[94mNN       \x1b[95mk:cccccccd        
      OoooooooooO       \x1b[94mXKX      \x1b[95mx:loooooooX       
      xoooooooood     \x1b[94mXKKKXXN    \x1b[95ml:ccloooooO       
      xlllllcclllO   \x1b[94mXKKKKKKX   \x1b[95mk:::::cllclO       
      0oooool:ooooK   \x1b[94mNXKKXN   \x1b[95mOccccccccl:oX       
      Ndooool:oooooO         Nxcccccccccc:x        
       Xolllc;cllllclO     Nkl:::::::::::cX        
        Xdllllllllll:llxK0xccccccc;cccccoX         
          Oooooooooo:oolcccccccccc:cccckN          
           Nkooooool:loolccccccccc;clxX            
             NOdllllllllllc::::::ldOX              
                 X0Okxddddoldxk0KN

              \x1b[95m[\x1b[97mTest This WAF - v1.0\x1b[95m]
                  \x1b[95m[\x1b[97mby AmoloHT\x1b[95m]
");
}

fn help() {
    println!("usage: ttwaf [-h] --all ALL --bypassed VALIDS

    options:
      -h, --help            show this help message and exit
      --all ALL,            show all
      --bypassed VALIDS     show only valid ones");
}

fn make_req() -> Result<(), Box<dyn Error>> {
    struct WAFs {
        cloudflare: String,
        akamai: String,
        aws: String,
        imperva: String,
        f5: String,
        cloudflare_error: String,
        akamai_error: String,
        aws_error: String,
        imperva_error: String,
        f5_error: String,
    }
    let waf = WAFs {
        cloudflare: String::from("https://www.cloudflare.com/favicon.ico"),
        akamai: String::from("https://www.akamai.com/site/favicon/favicon.ico"),
        aws: String::from("https://docs.aws.amazon.com/favicon.ico"),
        imperva: String::from("https://www.imperva.com/favicon.ico"),
        f5: String::from("https://www.f5.com/favicon.ico"),
        cloudflare_error: String::from("Attention Required!"),
        akamai_error: String::from("Denied"),
        aws_error: String::from("Request blocked"),
        imperva_error: String::from("unsuccessful"),
        f5_error: String::from("Request Rejected"),
    };


    let stdin = io::stdin();
    //let args: Vec<String> = env::args();
    let client = reqwest::blocking::Client::builder()
    .timeout(Duration::from_secs(15))
    .build()?;

    for arg in env::args() {
        if arg == "-h" || arg == "--help" {
            help();
        }

        if arg == "--bypassed" {
            for line in stdin.lock().lines() {
                let line = line.expect("\x1b[91m[-] \x1b[97mCould not read stdin input");
                let url_akamai = reqwest::Url::parse_with_params(&waf.akamai, [("a", &line)])?;
                let r_akamai = client.get(url_akamai).send()?;
                let r_akamai_text = r_akamai.text().unwrap();

                let url_cloudflare = reqwest::Url::parse_with_params(&waf.cloudflare, [("a", &line)])?;
                let r_cloudflare = client.get(url_cloudflare).send()?;
                let r_cloudflare_text = r_cloudflare.text().unwrap();

                let url_aws = reqwest::Url::parse_with_params(&waf.aws, [("a", &line)])?;
                let r_aws = client.get(url_aws).send()?;
                let r_aws_text = r_aws.text().unwrap();
                //println!("{}", r_aws_text);

                /*let url_imperva = reqwest::Url::parse_with_params(&waf.imperva, [("a", &line)])?;
                let r_imperva = client.get(url_imperva).send()?;
                let r_imperva_text = r_imperva.text().unwrap();*/

                let url_f5 = reqwest::Url::parse_with_params(&waf.f5, [("a", &line)])?;
                let r_f5 = client.get(url_f5).send()?;
                let r_f5_text = r_f5.text().unwrap();

                if !r_akamai_text.contains(&waf.akamai_error) {
                    println!("\x1b[92m[+] \x1b[97mAkamai: Accepted [{}]", &line);
                }

                if !r_aws_text.contains(&waf.aws_error) {
                    println!("\x1b[92m[+] \x1b[97mAWS: Accepted [{}]", &line);
                }

                if !r_cloudflare_text.contains(&waf.cloudflare_error) {
                    println!("\x1b[92m[+] \x1b[97mCloudflare: Accepted [{}]", &line);
                }

                /*if !r_imperva_text.contains(&waf.imperva_error) {
                    println!("\x1b[92m[+] \x1b[97mImperva: Accepted [{}]", &line);
                }*/

                if !r_f5_text.contains(&waf.f5_error) {
                    println!("\x1b[92m[+] \x1b[97mF5: Accepted [{}]", &line);
                }
            }
        }
        if arg == "--all" {
            for line in stdin.lock().lines() {
                let line = line.expect("\x1b[91m[-] \x1b[97mCould not read stdin input");
                let url_akamai = reqwest::Url::parse_with_params(&waf.akamai, [("a", &line)])?;
                let r_akamai = client.get(url_akamai).send()?;
                let r_akamai_text = r_akamai.text().unwrap();

                let url_cloudflare = reqwest::Url::parse_with_params(&waf.cloudflare, [("a", &line)])?;
                let r_cloudflare = client.get(url_cloudflare).send()?;
                let r_cloudflare_text = r_cloudflare.text().unwrap();

                let url_aws = reqwest::Url::parse_with_params(&waf.aws, [("a", &line)])?;
                let r_aws = client.get(url_aws).send()?;
                let r_aws_text = r_aws.text().unwrap();
                //println!("{}", r_aws_text);

                /*let url_imperva = reqwest::Url::parse_with_params(&waf.imperva, [("a", &line)])?;
                let r_imperva = client.get(url_imperva).send()?;
                let r_imperva_text = r_imperva.text().unwrap();*/

                let url_f5 = reqwest::Url::parse_with_params(&waf.f5, [("a", &line)])?;
                let r_f5 = client.get(url_f5).send()?;
                let r_f5_text = r_f5.text().unwrap();

                if r_akamai_text.contains(&waf.akamai_error) {
                    println!("\x1b[91m[-] \x1b[97mAkamai: Denied [{}]", &line);
                } else {
                    println!("\x1b[92m[+] \x1b[97mAkamai: Accepted [{}]", &line);
                }

                if r_aws_text.contains(&waf.aws_error) {
                    println!("\x1b[91m\x1b[91m[-] \x1b[97m\x1b[97mAWS: Denied [{}]", &line);   
                } else {
                    println!("\x1b[92m[+] \x1b[97mAWS: Accepted [{}]", &line);
                }

                if r_cloudflare_text.contains(&waf.cloudflare_error) {
                    println!("\x1b[91m[-] \x1b[97mCloudflare: Denied [{}]", &line);   
                } else {
                    println!("\x1b[92m[+] \x1b[97mCloudflare: Accepted [{}]", &line);
                }

                /*if r_imperva_text.contains(&waf.imperva_error) {
                    println!("\x1b[91m\x1b[91m[-] \x1b[97m\x1b[97mImperva: Denied [{}]", &line);   
                } else {
                    println!("\x1b[92m[+] \x1b[97mImperva: Accepted [{}]", &line);
                }*/

                if r_f5_text.contains(&waf.f5_error) {
                    println!("\x1b[91m\x1b[91m[-] \x1b[97m\x1b[97mF5: Denied [{}]", &line);   
                } else {
                    println!("\x1b[92m[+] \x1b[97mF5: Accepted [{}]", &line);
                }
            }
        }
    }
    //help();
    Ok(())
}

fn main() {
    banner();
    //make_req();
    loop {
        if let Err(_) = make_req() {
            println!("\x1b[91m[-] \x1b[97mThere was a problem");
        }
        break;
    }
}
