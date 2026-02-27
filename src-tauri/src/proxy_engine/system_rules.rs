use anyhow::Result;
use log::{error, info};
use tokio::process::Command;

const ROUTING_TABLE: u32 = 100;
const TPROXY_MARK: u32 = 1;

/// Setup iptables rules and routing policies to route traffic to the transparent proxy
pub async fn setup_tproxy_rules(local_port: u16, fwmark: u32) -> Result<()> {
    // Pre-cleanup: remove stale rules from previous crash/restart
    cleanup_tproxy_rules().await.ok();

    info!(
        "Setting up TPROXY rules for port {} skipping egress fwmark {:#x}",
        local_port, fwmark
    );

    // 1. IP Route configuration (Policy Routing)
    // Send packets marked with TPROXY_MARK to loopback interface
    run_cmd(
        "ip",
        &[
            "rule",
            "add",
            "fwmark",
            &TPROXY_MARK.to_string(),
            "table",
            &ROUTING_TABLE.to_string(),
        ],
    )
    .await
    .ok();
    run_cmd(
        "ip",
        &[
            "route",
            "add",
            "local",
            "0.0.0.0/0",
            "dev",
            "lo",
            "table",
            &ROUTING_TABLE.to_string(),
        ],
    )
    .await
    .ok();

    // 2. Mangle table rules for PREROUTING (TPROXY for external traffic like LAN devices if router)
    run_cmd("iptables", &["-t", "mangle", "-N", "PHOTON_PROXY"])
        .await
        .ok();
    run_cmd("iptables", &["-t", "mangle", "-F", "PHOTON_PROXY"])
        .await
        .ok();
    run_cmd(
        "iptables",
        &[
            "-t",
            "mangle",
            "-A",
            "PHOTON_PROXY",
            "-j",
            "RETURN",
            "-m",
            "mark",
            "--mark",
            &fwmark.to_string(),
        ],
    )
    .await?;
    ignore_private_ips("mangle", "PHOTON_PROXY").await?;
    run_cmd(
        "iptables",
        &[
            "-t",
            "mangle",
            "-A",
            "PHOTON_PROXY",
            "-p",
            "tcp",
            "-j",
            "TPROXY",
            "--on-port",
            &local_port.to_string(),
            "--tproxy-mark",
            &TPROXY_MARK.to_string(),
        ],
    )
    .await?;
    run_cmd(
        "iptables",
        &[
            "-t",
            "mangle",
            "-A",
            "PHOTON_PROXY",
            "-p",
            "udp",
            "-j",
            "TPROXY",
            "--on-port",
            &local_port.to_string(),
            "--tproxy-mark",
            &TPROXY_MARK.to_string(),
        ],
    )
    .await?;
    run_cmd(
        "iptables",
        &["-t", "mangle", "-A", "PREROUTING", "-j", "PHOTON_PROXY"],
    )
    .await?;

    // 3. Mangle table rules for OUTPUT (Host's own local traffic)
    run_cmd("iptables", &["-t", "mangle", "-N", "PHOTON_PROXY_MARK"])
        .await
        .ok();
    run_cmd("iptables", &["-t", "mangle", "-F", "PHOTON_PROXY_MARK"])
        .await
        .ok();
    run_cmd(
        "iptables",
        &[
            "-t",
            "mangle",
            "-A",
            "PHOTON_PROXY_MARK",
            "-j",
            "RETURN",
            "-m",
            "mark",
            "--mark",
            &fwmark.to_string(),
        ],
    )
    .await?;
    ignore_private_ips("mangle", "PHOTON_PROXY_MARK").await?;
    run_cmd(
        "iptables",
        &[
            "-t",
            "mangle",
            "-A",
            "PHOTON_PROXY_MARK",
            "-p",
            "tcp",
            "-j",
            "MARK",
            "--set-mark",
            &TPROXY_MARK.to_string(),
        ],
    )
    .await?;
    run_cmd(
        "iptables",
        &[
            "-t",
            "mangle",
            "-A",
            "PHOTON_PROXY_MARK",
            "-p",
            "udp",
            "-j",
            "MARK",
            "--set-mark",
            &TPROXY_MARK.to_string(),
        ],
    )
    .await?;
    run_cmd(
        "iptables",
        &["-t", "mangle", "-A", "OUTPUT", "-j", "PHOTON_PROXY_MARK"],
    )
    .await?;

    // 4. NAT table REDIRECT for Docker bridges (Mitigating bridge-netfilter bug)
    run_cmd("iptables", &["-t", "nat", "-N", "PHOTON_PROXY_DOCKER"])
        .await
        .ok();
    run_cmd("iptables", &["-t", "nat", "-F", "PHOTON_PROXY_DOCKER"])
        .await
        .ok();
    ignore_private_ips("nat", "PHOTON_PROXY_DOCKER").await?;
    run_cmd(
        "iptables",
        &[
            "-t",
            "nat",
            "-A",
            "PHOTON_PROXY_DOCKER",
            "-j",
            "RETURN",
            "-m",
            "mark",
            "--mark",
            &fwmark.to_string(),
        ],
    )
    .await?;
    run_cmd(
        "iptables",
        &[
            "-t",
            "nat",
            "-A",
            "PHOTON_PROXY_DOCKER",
            "-i",
            "docker0",
            "-p",
            "tcp",
            "-j",
            "REDIRECT",
            "--to-ports",
            &local_port.to_string(),
        ],
    )
    .await?;
    run_cmd(
        "iptables",
        &[
            "-t",
            "nat",
            "-A",
            "PHOTON_PROXY_DOCKER",
            "-i",
            "br-public",
            "-p",
            "tcp",
            "-j",
            "REDIRECT",
            "--to-ports",
            &local_port.to_string(),
        ],
    )
    .await
    .ok(); // generic
    run_cmd(
        "iptables",
        &["-t", "nat", "-A", "PREROUTING", "-j", "PHOTON_PROXY_DOCKER"],
    )
    .await?;

    Ok(())
}

/// Clean up system rules previously created
pub async fn cleanup_tproxy_rules() -> Result<()> {
    info!("Cleaning up TPROXY iptables and routing rules");

    run_cmd(
        "iptables",
        &["-t", "mangle", "-D", "PREROUTING", "-j", "PHOTON_PROXY"],
    )
    .await
    .ok();
    run_cmd("iptables", &["-t", "mangle", "-F", "PHOTON_PROXY"])
        .await
        .ok();
    run_cmd("iptables", &["-t", "mangle", "-X", "PHOTON_PROXY"])
        .await
        .ok();

    run_cmd(
        "iptables",
        &["-t", "mangle", "-D", "OUTPUT", "-j", "PHOTON_PROXY_MARK"],
    )
    .await
    .ok();
    run_cmd("iptables", &["-t", "mangle", "-F", "PHOTON_PROXY_MARK"])
        .await
        .ok();
    run_cmd("iptables", &["-t", "mangle", "-X", "PHOTON_PROXY_MARK"])
        .await
        .ok();

    run_cmd(
        "iptables",
        &["-t", "nat", "-D", "PREROUTING", "-j", "PHOTON_PROXY_DOCKER"],
    )
    .await
    .ok();
    run_cmd("iptables", &["-t", "nat", "-F", "PHOTON_PROXY_DOCKER"])
        .await
        .ok();
    run_cmd("iptables", &["-t", "nat", "-X", "PHOTON_PROXY_DOCKER"])
        .await
        .ok();

    run_cmd(
        "ip",
        &[
            "rule",
            "del",
            "fwmark",
            &TPROXY_MARK.to_string(),
            "table",
            &ROUTING_TABLE.to_string(),
        ],
    )
    .await
    .ok();
    run_cmd(
        "ip",
        &[
            "route",
            "del",
            "local",
            "0.0.0.0/0",
            "dev",
            "lo",
            "table",
            &ROUTING_TABLE.to_string(),
        ],
    )
    .await
    .ok();

    Ok(())
}

async fn ignore_private_ips(table: &str, chain: &str) -> Result<()> {
    let subnets = [
        "127.0.0.0/8",
        "192.168.0.0/16",
        "10.0.0.0/8",
        "172.16.0.0/12",
        "169.254.0.0/16", // Link-local (Docker metadata, APIPA)
        "224.0.0.0/4",    // Multicast
    ];
    for subnet in subnets {
        run_cmd(
            "iptables",
            &["-t", table, "-A", chain, "-d", subnet, "-j", "RETURN"],
        )
        .await?;
    }
    Ok(())
}

async fn run_cmd(cmd: &str, args: &[&str]) -> Result<()> {
    let output = Command::new(cmd).args(args).output().await?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        error!("Command failed: {} {:?}. Error: {}", cmd, args, err);
        anyhow::bail!("Command error: {}", err);
    }
    Ok(())
}
