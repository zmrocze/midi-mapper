use std::io::Write;
use std::process::Command;
use std::process::Stdio;

#[derive(Debug)]
pub enum Error {
  ProcessIo(std::io::Error),
  ExitCode(std::process::ExitStatus),
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Error {
    Error::ProcessIo(err)
  }
}

impl From<std::process::ExitStatus> for Error {
  fn from(err: std::process::ExitStatus) -> Error {
    Error::ExitCode(err)
  }
}

pub fn call_process(mut cmd: Command, stdin: Vec<u8>) -> Result<Vec<u8>, Error> {
  let mut cmd = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
  let mut input = cmd.stdin.take().unwrap();
  input.write_all(&stdin)?;
  input.flush()?;
  drop(input);
  let out = cmd.wait_with_output()?;
  if out.status.success() {
    return Ok(out.stdout);
  } else {
    return Err(Error::ExitCode(out.status));
  }
}

pub fn call_dhall_to_yaml(dhall: Vec<u8>) -> Result<Vec<u8>, Error> {
  let cmd = Command::new("dhall-to-yaml-ng");
  call_process(cmd, dhall)
}
