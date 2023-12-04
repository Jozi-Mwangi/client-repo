use std::net::{TcpStream, Shutdown};
use std::io::{ Read, Write };
use std::fs;
use client_side::encode_to_base64;
use env_logger::{Builder, Env};
use log::{error};

fn init_logger(){
    let env = Env::default()
        .filter("println");

    Builder::from_env(env)
        .format_level(false)
        .format_timestamp_nanos()
        .init();
}

fn main () {
    init_logger();
    // std::env::set_var("RUST_LOG", "println");


   let report_file_path = "./data/data/CTONGA/branch_weekly_sales.txt";
    start_data_transfer("127.0.0.1:8080", "CTONGA", report_file_path)
}

fn start_data_transfer(server_address: &str, branch_code: &str, report_file_path: &str){


    // Reading the file contents and encoding it to Base64
    let file_content = fs::read_to_string(report_file_path)
        .expect("Error reading the sales report");
    // println!("{}", file_content);


    let base64_content = encode_to_base64(&file_content);
    // println!("Encoded file: {}", base64_content);

    // Connecting to the server
    match TcpStream::connect(server_address)  {
        Ok(mut stream) => {
            println!("Connected to Server");

            // Send branch code back to the server
            let branch_code = format!("bcode~{}", branch_code);
            let branch_length = branch_code.len();
            if let Err(e) = stream.write_all(&(branch_length as u32).to_be_bytes()) {
                error!("Error writing the length: {:?}", e)
            }
            if let Err(e) = stream.write_all(branch_code.as_bytes()){
                error!("Error writing branch message: {:?}", e);
                // return;
            }else {
                println!("Branch code sent successfully")
            };
            println!("Branch code: {}", branch_code);
    
            // Receive an acknowldgement from the server
            // Reading the "OK" lenght
            let mut resp_length_buff = [0; 4];
            if let Err(e) = stream.read_exact(&mut resp_length_buff){
                error!("Error reading response: {:?} ", e)
            };
            let length = u32::from_be_bytes(resp_length_buff);
            
            // Reading the "Ok" response
            let mut response_buff = vec![0; length as usize];
            stream.read_exact(&mut response_buff).expect("Failed to read response");
            let response = String::from_utf8_lossy(&response_buff);
            println!("1st response: {}", response);
            
            if response.trim() == "OK" {
                // Send base64 content to the server
                let file_message = format!("~{}~", base64_content);
                if let Err(e) = stream.write(file_message.as_bytes()){
                    error!("Error writing file message to stream: {:?}", e);
                    // return;
                };
    
                // Receive an acknowledgement from the server
                let mut response = String::new();
                if let Err(e) = stream.read_to_string(&mut response){
                    error!("Error reading response from stream: {:?}", e)
                };
    
                if response.trim() == "OK"{
                    println!("Sales report transferred successfully")
                } else {
                    error!("Error in sales transfer")
                }
            } else {
                error!("Error in branch code acknowledgement")
            }

            if stream.shutdown(Shutdown::Both).is_err(){
                error!("Failed to close the connection")
            }
        }
        Err(err) => {
            error!("Failed to connect to server: {:?}", err);
            // return;
        }
    } 

}
