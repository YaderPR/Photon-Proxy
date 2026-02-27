# Assistant Agent Rules

## Strict Prohibition of Background `sudo` Execution

**It is strictly forbidden for the Assistant Agent to execute commands requiring `sudo` in the background (using `&` or `nohup`) or within non-interactive scripts (such as using `cat << EOF > script.sh` containing `sudo`).**

### Reason for Incident
The user's Linux system has built-in brute-force protection mechanisms (`pam_faillock` or `pam_tally2`). By sending commands like `sudo ./src-tauri/target/debug/photon-proxy &` to the background without an attached interactive terminal (TTY), the process repeatedly attempted to request a password from an empty standard input (stdin). This generated multiple failed authentication attempts per second, resulting in a temporary system-level lock (ban) of the user's account (`yaderpr`).

### Rules to Follow:
1. **Never** start the proxy binary with `sudo` in the background during testing.
2. If a command requires `sudo`, the Agent must **explicitly ask the user** to run it manually in their own terminal.
3. The Agent may only execute tools that do not require privilege escalation in the automated environment (`cargo check`, `cargo build`, local scripts without sudo).
4. The `./setup-permissions.sh` script is a managed partial exception, but injecting raw `sudo` commands into the automation flow must be avoided unless the user has configured passwordless sudo (`NOPASSWD`).

*This document was generated at the express request of the user to prevent future system lockouts.*
