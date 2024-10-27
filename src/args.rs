use clap::{Parser};

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct SlurmArgs {
    /// please provide the script for the submission to the slurm
    pub script_arg: String,
    /// please provide the username for the server
    pub user_arg: String,
    /// please provide the password for the server
    pub passwd_arg: String,
    /// please provide the path to the directory on the host
    pub hostdir_arg: String,
}
