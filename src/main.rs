mod args;
use std::env::Args;
use dotenv_vault::*;
use std::sync::mpsc::channel;
use clap::Parser;
use args::SlurmArgs;
use ssh::{self, LocalShell, SshError};
use std::time::Duration;

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
    let user = std::env::var("USER").unwrap_or("".to_string());
    let ip = std::env::var("IP").unwrap_or("".to_string());
    let pubkey = std::env::var("PUB").unwrap_or("".to_string());
    let script = args.script_arg.to_string();
    let combined = format!("{}/{}",args.hostdir_arg,args.script_arg).to_string();

    //opening a session

    let mut session = ssh::create_session()
    .username(args.username_arg)
    .private_key_path(pubkey)
    .connect(ip)
    .unwrap();

    // sending the file to the server
    let scp = session.open_scp().unwrap();
    scp.upload(args.hostdir_arg, args.script_arg).unwrap();
    assert_file(&combined);
    let exec = session.open_exec().unwrap();

    // submitting the job to the cluster and closing the cluster.
    let vec: Vec<u8> = exec.send_command("sbatch &combined").unwrap();
    println!("{}", String::from_utf8(vec).unwrap());
    println!("exit status: {}", exec.exit_status().unwrap());
    println!("terminated msg: {}", exec.terminate_msg().unwrap());
    let _ = exec.close();
    session.close();
    let printoutput = String::from("job_submitted");
    println!("{}", &printoutput);
}
