
use clap::ArgMatches;

use config::Config;

#[derive(Debug)]
pub struct DropletConfig {
    name: String,
    region: String,
    size: String,
    image: String,
    ssh_keys: Option<Vec<String>>,
    backups: bool,
    ipv6: bool,
    private_net: bool,
    data: Option<String>
}

impl DropletConfig {
    pub fn from_matches(m: &ArgMatches) -> DropletConfig {
        DropletConfig {
            name: m.value_of("name").unwrap().to_owned(),
            region: m.value_of("region").unwrap().to_owned(),
            size: m.value_of("size").unwrap().to_owned(),
            image: m.value_of("image").unwrap().to_owned(),
            ssh_keys: if let Some(v) = m.values_of("keys") {
                Some(v.iter().map(|&k| k.to_owned()).collect::<Vec<_>>())
            } else {
                None
            },
            backups: m.is_present("backups"),
            ipv6: m.is_present("ipv6"),
            private_net: m.is_present("private-networking"),
            data: if let Some(d) = m.value_of("data") {
                Some(d.to_owned())
            } else {
                None
            }
        }
    }
}

pub fn run(m: &ArgMatches, cfg: &Config) {
    let id = m.value_of("id").unwrap();
    match m.subcommand() {
        ("", _)                      => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).retrieve_json() {
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
            CliMessage::Droplet(id).display();
            match domgr.droplet(id).retrieve() {
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
        ("list-kernels", _)              => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                         .kernels()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).kernels().retrieve_json() {
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
            CliMessage::DropletKernels(id).display();
            match domgr.droplet(id).kernels().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v.iter() {
                        CliMessage::Kernel.display();
                        println!("\t{}", act);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        ("list-snapshots", _)            => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                         .snapshots()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).snapshots().retrieve_json() {
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
            CliMessage::DropletKernels(id).display();
            match domgr.droplet(id).snapshots().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v.iter() {
                        CliMessage::Snapshot.display();
                        println!("\t{}", act);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        ("list-backups", _)              => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                         .backups()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).backups().retrieve_json() {
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
            CliMessage::DropletBackups(id).display();
            match domgr.droplet(id).backups().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v.iter() {
                        CliMessage::Backup.display();
                        println!("\t{}", act);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        ("list-actions", _)              => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                         .actions()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).actions().retrieve_json() {
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
            CliMessage::DropletActions(id).display();
            match domgr.droplet(id).actions().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v.iter() {
                        CliMessage::Action.display();
                        println!("\t{}", act);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        ("delete", _)                    => {
            println!("Deleting droplet with id: {}", id);
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .delete()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).delete().retrieve_json() {
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
            CliMessage::DeleteDroplet(id).display();
            match domgr.droplet(id).delete().retrieve() {
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
        ("list-neighbors", _)            => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                         .neighbors()
                         .to_string()
                         .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).neighbors().retrieve_json() {
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
            CliMessage::DropletNeighbors(id).display();
            match domgr.droplet(id).neighbors().retrieve() {
                Ok(v) => {
                    CliMessage::Success.display();
                    for act in v.iter() {
                        CliMessage::Neighbor.display();
                        println!("\t{}", act);
                    }
                },
                Err(e) => {
                    CliMessage::Failure.display();
                    println!("{}\n", e);
                }
            }
        },
        ("disable-backups", _)           => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .disable_backups()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).disable_backups().retrieve_json() {
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
            CliMessage::DisableBackups(id).display();
            match domgr.droplet(id).disable_backups().retrieve() {
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
        ("reboot", _)                    => {
            println!("Rebooting droplet with id: {}", id);
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .reboot()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).reboot().retrieve_json() {
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
            CliMessage::RebootDroplet(id).display();
            match domgr.droplet(id).reboot().retrieve() {
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
        ("power-cycle", _)               => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .power_cycle()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).power_cycle().retrieve_json() {
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
            CliMessage::PowerCycleDroplet(id).display();
            match domgr.droplet(id).power_cycle().retrieve() {
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
        ("shutdown", _)                  => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .shutdown()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).shutdown().retrieve_json() {
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
            CliMessage::ShutdownDroplet(id).display();
            match domgr.droplet(id).shutdown().retrieve() {
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
        ("power-off", _)                 => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .power_off()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).power_off().retrieve_json() {
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
            CliMessage::PowerOffDroplet(id).display();
            match domgr.droplet(id).power_off().retrieve() {
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
        ("power-on", _)                  => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .power_on()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).power_on().retrieve_json() {
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
            CliMessage::PowerOnDroplet(id).display();
            match domgr.droplet(id).power_on().retrieve() {
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
        ("restore", Some(m))             => {
            let img = m.value_of("image").unwrap();
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .restore(img)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).restore(img).retrieve_json() {
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
            CliMessage::RestoreDroplet(id, img).display();
            match domgr.droplet(id).restore(img).retrieve() {
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
        ("reset-password", _)            => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .reset_password()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).reset_password().retrieve_json() {
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
            CliMessage::ResetPassword(id).display();
            match domgr.droplet(id).reset_password().retrieve() {
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
        ("resize", Some(m))              => {
            let disk = m.is_present("disk");
            let size = m.value_of("size").unwrap();
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .resize(size, disk)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).resize(size, disk).retrieve_json() {
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
            CliMessage::ResizeDroplet(id, size, disk).display();
            match domgr.droplet(id).resize(size, disk).retrieve() {
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
        ("rebuild", Some(m))             => {
            let img = m.value_of("image").unwrap();
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .rebuild(img)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).rebuild(img).retrieve_json() {
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
            CliMessage::RebuildDroplet(id, img).display();
            match domgr.droplet(id).rebuild(img).retrieve() {
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
        ("rename", Some(m))              => {
            let name = m.value_of("name").unwrap();
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .rename(name)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).rename(name).retrieve_json() {
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
            CliMessage::RenameDroplet(id, name).display();
            match domgr.droplet(id).rename(name).retrieve() {
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
        ("change-kernel", Some(m))       => {
            let kernel = m.value_of("kernel_id").unwrap();
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .change_kernel(kernel)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).change_kernel(kernel).retrieve_json() {
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
            CliMessage::ChangeKernel(id, kernel).display();
            match domgr.droplet(id).change_kernel(kernel).retrieve() {
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
        ("enable-ipv6", _)               => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .enable_ipv6()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).enable_ipv6().retrieve_json() {
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
            CliMessage::EnableIpv6(id).display();
            match domgr.droplet(id).enable_ipv6().retrieve() {
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
        ("enable-private-networking", _) => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .enable_private_networking()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).enable_private_networking().retrieve_json() {
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
            CliMessage::EnablePrivateNetworking(id).display();
            match domgr.droplet(id).enable_private_networking().retrieve() {
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
        ("snapshot", _)                  => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .snapshot()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).snapshot().retrieve_json() {
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
            CliMessage::SnapshotDroplet(id).display();
            match domgr.droplet(id).snapshot().retrieve() {
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
        ("show-action", Some(m))         => {
            let a_id = m.value_of("action_id").unwrap();
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .action(a_id)
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).action(a_id).retrieve_json() {
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
            CliMessage::DropletAction(id, a_id).display();
            match domgr.droplet(id).action(a_id).retrieve() {
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
        ("upgrade", _)                   => {
            if cfg.debug {
                CliMessage::Request(
                    &domgr.droplet(id)
                          .upgrade()
                          .to_string()
                          .replace("\n", "\n\t")[..]).display();
            }
            if cfg.no_send { return }
            if cfg.debug {
                CliMessage::JsonResponse.display();
                match domgr.droplet(id).upgrade().retrieve_json() {
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
            CliMessage::UpgradeDroplet(id).display();
            match domgr.droplet(id).upgrade().retrieve() {
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
        _                                => unreachable!()
    }
}
