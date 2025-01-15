# rust-connect 

- a rustlang devops/system administration application for automatically submitting jobs to slurm schedulers. 
- provide the pub file, username and the ipaddress and the script to be submitted to the cluster.
- an implementation of the server key algorithm.
- please see the last commit message and if it says compiled binary then it is completed or else still in development version.

```
cargo build 
╭─gauavsablok@gauravsablok ~/Desktop/rust/rust-connect/target/debug ‹main●›
╰─$ ./rust-connect -h
Usage: rust-connect <SCRIPT_ARG> <USER_ARG> <PASSWD_ARG> <HOSTDIR_ARG>

Arguments:
  <SCRIPT_ARG>   please provide the script for the submission to the slurm
  <USER_ARG>     please provide the username for the server
  <PASSWD_ARG>   please provide the password for the server
  <HOSTDIR_ARG>  please provide the path to the directory on the host

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Gaurav Sablok
