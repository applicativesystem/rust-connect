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
 * Just specify the rsa file which should be the connect file for the slurm along with
 * y:our username and the password and the path on your computer for the submission script.
 *
 * It implements the Server host key algorithm
 *
 * */


fn main() {

    let args:SlurmArgs = SlurmArgs::parse();
     // reading the dotenv file and merging the same with the format! as the combined one for
     // submission.
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
    channel.exec(&combined);
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());
}
