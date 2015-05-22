
use clap::ArgMatches;

use config::Config;
use cli::list;

pub fn run(m: &ArgMatches, cfg: &Config) {
    match m.subcommand() {
        ("create", Some(m))   => {
            let name = m.value_of("name").unwrap();
            let pub_key = m.value_of("public_key").unwrap();
            if cfg.debug {
                CliMessage::Request(
                    &domgr.ssh_keys()
                          .create(name, pub_key)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.ssh_keys().create(name, pub_key).retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::CreateSshKey(name, pub_key).display();
            match domgr.ssh_keys().create(name, pub_key).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("show-key", Some(m)) => {
            let id = m.value_of("id").unwrap();
            let finger = m.value_of("finger_print").unwrap();
            if cfg.debug {
                CliMessage::Request(
                    &domgr.ssh_keys()
                          .show(name, finger)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.ssh_keys().show(name, finger).retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::SshKey(name, finger).display();
            match domgr.ssh_keys().show(name, finger).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("update", Some(m))   => {
            let finger = m.is_present("finger_print");
            let id = if m.is_present("id") {
                m.value_of("id").unwrap()
            } else {
                m.value_of("finger_print").unwrap()
            };
            let name = m.value_of("name").unwrap();
            if cfg.debug {
                CliMessage::Request(
                    &domgr.ssh_keys()
                          .update(name, id)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.ssh_keys().update(name, id).retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::UpdateSshKey(name, id).display();
            match domgr.ssh_keys().update(name, id).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("destroy", Some(m))  => {
            let id = if m.is_present("id") {
                m.value_of("id").unwrap()
            } else {
                m.value_of("finger_print").unwrap()
            };
            if cfg.debug {
                CliMessage::Request(
                    &domgr.ssh_keys()
                          .destroy(id)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.ssh_keys().destroy(id).retrieve_json() {
                    Ok(s) => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::DestroySshKey(id).display();
            match domgr.ssh_keys().destroy(id).retrieve() {
                Ok(s) => {
                    CliMessage::Success.display();
                    println!("\n\t{}\n", s);
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("\n\t{}\n", e);
                }
            }
        },
        ("", _) => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.ssh_keys()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.ssh_keys().retrieve_json() {
                    Ok(s)  => {
                        CliMessage::Success.display();
                        println!("\n\t{}\n", s);
                    },
                    Err(e) => {
                        CliMessage::Failure.display();
                        println!("\n\t{}\n", e);
                    }
                }
            }
            CliMessage::SshKeys.display();
            match domgr.ssh_keys().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for k in v.iter() {
                        CliMessage::SshKeys.display();
                        println!("\t{}", k);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        _          => unreachable!()
    }
}
