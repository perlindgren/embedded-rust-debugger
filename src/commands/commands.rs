use super::{
    Command,
    debug_request::DebugRequest,
};

use anyhow::{ Result, anyhow };

use std::path::PathBuf;



struct CommandInfo {
    pub name:           &'static str,
    pub description:    &'static str,
    pub parser: fn(args: &[&str]) -> Result<DebugRequest>,
}


pub struct Commands {
    commands:   Vec<CommandInfo>,
}

impl Commands {
    pub fn new() -> Commands {
        Commands {
            commands: vec!(
                CommandInfo {
                    name: "attach",
                    description: "Set the current work directory",
                    parser: |_args| {
                        Ok(DebugRequest::Attach {// TODO: Parse arguments
                            reset: false,
                            reset_and_halt: false,
                        })
                    },
                },
                CommandInfo {
                    name: "set-work-directory",
                    description: "Set the current work directory",
                    parser: |args| {
                        if args.len() > 0 {
                            return Ok(DebugRequest::SetCWD {
                                cwd: args[0].to_string(),
                            });
                        }
                        Err(anyhow!("Requires a string as a argument"))
                    },
                },
                CommandInfo {
                    name: "stack",
                    description: "Prints the current stack values",
                    parser: |_args| {
                        Ok(DebugRequest::Stack)
                    },
                },
                CommandInfo {
                    name: "code",
                    description: "Prints the current code",
                    parser: |_args| {
                        Ok(DebugRequest::Code)
                    },
                },
                CommandInfo {
                    name: "clear-all-breakpoints",
                    description: "Removes all hardware breakpoints",
                    parser: |_args| {
                        Ok(DebugRequest::ClearAllBreakpoints)
                    },
                },
                CommandInfo {
                    name: "clear-breakpoint",
                    description: "Remove a hardware breakpoint",
                    parser: |args| {
                        if args.len() > 0 {
                            let address = parse_u32_from_str(args[0])?;
                            return Ok(DebugRequest::ClearBreakpoint {
                                address: address,
                            });
                        }
                        Err(anyhow!("Requires a string as a argument"))
                    },
                },
                CommandInfo {
                    name: "set-breakpoint",
                    description: "Set a hardware breakpoint",
                    parser: |args| {
                        if args.len() > 0 {
                            let address = parse_u32_from_str(args[0])?;
                            let path = match args.len() {
                                2 => Some(args[1].to_string()),
                                _ => None,
                            };

                            return Ok(DebugRequest::SetBreakpoint {
                                address: address,
                                source_file: path,
                            });
                        }
                        Err(anyhow!("Requires a string as a argument"))
                    },
                },
                CommandInfo {
                    name: "registers",
                    description: "Print all register values",
                    parser: |_args| {
                        Ok(DebugRequest::Registers)
                    },
                },
                CommandInfo {
                    name: "variable",
                    description: "Print the value of a variable",
                    parser: |args| {
                        if args.len() > 0 {
                            let name = args[0].to_string();
                            return Ok(DebugRequest::Variable {
                                name: name,
                            });
                        }
                        Err(anyhow!("Requires a string as a argument"))
                    },
                },
                CommandInfo {
                    name: "variables",
                    description: "Print all local variables",
                    parser: |_args| {
                        Ok(DebugRequest::Variables)
                    },
                },
                CommandInfo {
                    name: "set-chip",
                    description: "Set chip model being used",
                    parser: |args| {
                        if args.len() > 0 {
                            let chip = args[0].to_string();
                            return Ok(DebugRequest::SetChip {
                                chip: chip,
                            });
                        }
                        Err(anyhow!("Requires a string as a argument"))
                    },
                },
                CommandInfo {
                    name: "set-probe-number",
                    description: "Set the probe number to use",
                    parser: |args| {
                        if args.len() > 0 {
                            let number = parse_u32_from_str(args[0])? as usize;
                            return Ok(DebugRequest::SetProbeNumber {
                                number: number,
                            });
                        }
                        Err(anyhow!("Requires a boolean as a argument"))
                    },
                },
                CommandInfo {
                    name: "stack-trace",
                    description: "Print stack trace",
                    parser: |_args| {
                        Ok(DebugRequest::StackTrace)
                    },
                },
                CommandInfo {
                    name: "read",
                    description: "Read address in memory",
                    parser: |args| {
                        if args.len() > 0 {
                            let address = parse_u32_from_str(args[0])?;
                            let byte_size = match args.len() {
                                2 => parse_u32_from_str(args[1])?,
                                _ => 4,
                            } as usize;
                            return Ok(DebugRequest::Read {
                                address: address,
                                byte_size: byte_size,
                            });
                        }
                        Err(anyhow!("Requires a boolean as a argument"))
                    },
                },
                CommandInfo {
                    name: "reset",
                    description: "Reset or reset and halt the core",
                    parser: |args| {
                        let mut reset_and_halt = false;
                        if args.len() > 0 {
                            reset_and_halt = parse_bool(args[0])?;
                        }

                        Ok(DebugRequest::Reset {
                            reset_and_halt: reset_and_halt
                        })
                    },
                },
                CommandInfo {
                    name: "flash",
                    description: "Flash target with binary file",
                    parser: |args| {
                        let mut reset_and_halt = false;
                        if args.len() > 0 {
                            reset_and_halt = parse_bool(args[0])?;
                        }

                        Ok(DebugRequest::Flash { reset_and_halt: reset_and_halt })
                    },
                },
                CommandInfo {
                    name: "step",
                    description: "Step one assembly instruction",
                    parser: |_args| {
                        Ok(DebugRequest::Step)
                    },
                },
                CommandInfo {
                    name: "status",
                    description: "Print the status of the core",
                    parser: |_args| {
                        Ok(DebugRequest::Status)
                    },
                },
                CommandInfo {
                    name: "exit",
                    description: "Exit debugger",
                    parser: |_args| {
                        Ok(DebugRequest::Exit)
                    },
                },
                CommandInfo {
                    name: "continue",
                    description: "Continue the program",
                    parser: |_args| {
                        Ok(DebugRequest::Continue)
                    },
                },
                CommandInfo {
                    name: "halt",
                    description: "Halt the core",
                    parser: |_args| {
                        Ok(DebugRequest::Halt)
                    },
                },
                CommandInfo {
                    name: "set-binary",
                    description: "Set the binary file to debug",
                    parser: |args| {
                        if args.len() > 0 {
                            let path = PathBuf::from(args[0]);
                            return Ok(DebugRequest::SetBinary { path: path });
                        }
                        Err(anyhow!("Requires a path as a argument"))
                    },
                },
            ),
        }
    }


    pub fn parse_command(&self, line: &str) -> Result<Command> {
        let mut command_parts = line.split_whitespace();
        if let Some(command) = command_parts.next() {
            
            let cmd = self.commands.iter().find(|c| c.name == command);
            
            if let Some(cmd) = cmd {
                let remaining_args: Vec<&str> = command_parts.collect();

                return Ok(Command::Request((cmd.parser)(&remaining_args)?));
            } else {
                return Err(anyhow!("Unknown command '{}'\n\tEnter 'help' for a list of commands", command));
            }
        }

        Err(anyhow!("Empty Command"))
    }


    pub fn check_if_help(&self, line: &str) -> Option<String> {
        let mut command_parts = line.split_whitespace();
        if let Some(command) = command_parts.next() {
            if command == "help" {
                let mut help_string = format!("Available commands:");
                for cmd in &self.commands {
                    help_string = format!("{}\n\t- {}: {}",
                                          help_string,
                                          cmd.name,
                                          cmd.description);
                }
                return Some(help_string);
            }
        }

        None
    }
}


fn parse_u32_from_str(s: &str) -> Result<u32> {
    if s.starts_with("0x") {
        let without_prefix = s.trim_start_matches("0x");
        return Ok(u32::from_str_radix(without_prefix, 16)?);
    } else {
        return Ok(u32::from_str_radix(s, 10)?); 
    };
}


fn parse_bool(s: &str) -> Result<bool> {
    match s {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(anyhow!("Expected a boolean argument")),
    }
}

