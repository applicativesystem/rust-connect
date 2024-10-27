mod args;
use clap::Parser;
use args::SlurmArgs;
use std::net::TcpStream;
use ssh2::Session;
use std::io::prelude::*;

/*
 *Author Gaurav Sablok
 *Universitat Potsdam
 *Date: 2024-10-28
 * A rustlang based dotenv reader and job submission scheduler for slurm job submissions.
 * give your IP address in the dot env and the username and the password as command line
 * with the filename.
 *
 * It implements the Server host key algorithm
 *
 * */


fn main() {

    let args:SlurmArgs = SlurmArgs::parse();
    let _  = dotenv_vault::dotenv();
    let ip = std::env::var("IP").unwrap_or("".to_string());
    let user = args.user_arg.to_string();
    let password = args.passwd_arg.to_string();
    let combined = format!("scp{}@{}:{}/{}",user,ip,args.hostdir_arg,args.script_arg).to_string();
    let tcp = TcpStream::connect(ip).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&user, &password).unwrap();
    assert!(sess.authenticated());
    let mut channel = sess.channel_session().unwrap();
    let _ = channel.exec(&combined);
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());
}
