# macguard

**English** · [Tiếng Việt](README.vi.md)

A fast command-line tool that audits and hardens macOS, following the
[macOS Security and Privacy Guide](https://github.com/drduh/macOS-Security-and-Privacy-Guide) by drduh.

- **44 checks**: 40 come from the guide's checkable recommendations (disk encryption, boot security,
  updates, XProtect, backups, login, firewall, DNS, certificates, Wi-Fi, SSH, open ports, background
  items, system extensions, Homebrew and privacy defaults) and 4 are common hardening steps the guide
  does not cover. See [How closely it follows the guide](#how-closely-it-follows-the-guide).
- **Interactive menu** like [Mole](https://github.com/tw93/Mole)'s `mo`, sorted by priority: high,
  then medium, then low, with failing items first. Open the analysis, fix or ignore an item with one
  key. It quits only on `q` or Ctrl-C.
- **Every fix is explained before it runs**: what it fixes, the exact commands, what it does, the risk
  of not fixing it and the risk of fixing it. Nothing runs without your `y`.
- **Ignore what does not apply to you.** Ignored items are not checked next time.
- **English and Vietnamese**, chosen from your macOS language or `--lang en|vi`.
- A single native binary written in Rust. All checks run in parallel (about one second), the menu
  responds instantly, and the only network access is `macguard update` when you run it.

## Requirements

macOS 13 Ventura or later, Apple silicon or Intel. Some checks need `sudo` and are skipped without it.

## Install

Download the latest release (a universal binary for Apple silicon and Intel):

```bash
curl -fsSLO https://github.com/namtao/macguard/releases/latest/download/macguard-macos-universal.tar.gz
curl -fsSLO https://github.com/namtao/macguard/releases/latest/download/macguard-macos-universal.tar.gz.sha256
shasum -a 256 -c macguard-macos-universal.tar.gz.sha256
tar -xzf macguard-macos-universal.tar.gz
sudo install -m 755 macguard /usr/local/bin/macguard
```

Or build from source with Rust 1.80 or later:

```bash
cargo install --git https://github.com/namtao/macguard
```

### Update

```bash
macguard update          # use 'sudo macguard update' if the binary is in a folder you cannot write to
```

`macguard update` asks GitHub for the latest release, downloads it with `curl`, verifies its SHA-256
checksum and replaces the installed binary in one atomic step. If you installed with `cargo install`,
running `cargo install --git https://github.com/namtao/macguard --force` again also works.

### Uninstall

Delete the binary (`sudo rm /usr/local/bin/macguard`, or `cargo uninstall macguard`) and, if you used
the ignore list, `~/.config/macguard`.

## Usage

### Interactive menu

```bash
macguard          # open the menu
sudo macguard     # also check items that need root (firmware, sudoers, every open port)
```

The menu appears at once and fills in as the checks finish. Items are grouped by priority:

```text
macguard 1.0.0 · macOS 15.5
Score 28/34 · 2 high/medium failing
──────────────────────────────────────────────────────────────────────────────
 High priority
   ✓ filevault          FileVault disk encryption
   ✓ firewall           Application firewall is on
 Medium priority · 2 failing
   ✗ backup             Encrypted Time Machine backup (no backup disk)
 ▸ ✗ screenlock         Password required immediately after sleep or screen saver (delay 300 s)
   ✓ ssh                Remote Login (SSH server) is off
 Low priority · 3 failing
   ! dns                Encrypted or filtering DNS
   ⊘ bluetooth          Bluetooth is off when not needed
──────────────────────────────────────────────────────────────────────────────
↑↓ move · Enter analyse · f fix · i ignore/unignore · a failing only/all · r rescan · q quit
```

| Key | Action |
|---|---|
| `↑` `↓` (or `k` `j`), PgUp, PgDn, Home, End | Move |
| Enter (or `→`) | Open the analysis: effect, exact command, risk of fixing and of not fixing, source |
| `f` | Fix the selected item. The analysis is shown and macguard asks `y/N` before running anything |
| `i` | Ignore the item (it will not be checked next time), or check it again |
| `o` (in the analysis) | Open the matching pane of System Settings |
| `a` | Show only failing items, or all items |
| `r` | Scan again |
| `←` or Esc (in the analysis) | Back to the list |
| `q` or Ctrl-C | Quit |

### Commands for scripts

```bash
macguard check              # plain report by priority; exit code 1 when a high or medium item fails
macguard explain screenlock # analysis of one or more items
macguard fix                # walk through every failing item and ask before each fix
macguard fix airplay ssh    # fix only these items
macguard ignore bluetooth   # do not check this item any more
macguard unignore bluetooth # check it again
macguard list               # all item ids by priority (⊘ = ignored)
macguard tips               # guide advice that cannot be checked automatically
macguard update             # install the latest release
macguard --lang vi check    # Vietnamese output (or: export MACGUARD_LANG=vi)
```

Without a terminal (for example in a pipe or a cron job), `macguard` with no command prints the
report instead of opening the menu. The ignore list is stored per user in `~/.config/macguard/ignored`,
one id per line.

Marks: `✓` pass, red `✗` high, yellow `✗` medium, `!` low, `i` for review, `-` not applicable (with
the reason), `⊘` ignored, `…` still checking.

### What a fix shows before it runs

```text
✗ AirPlay Receiver is off (airplay, medium, Sharing and open ports)
  Status
    not passing
  What it does
    Stops Control Center from listening on ports 5000 and 7000 on every network interface.
  Command that will run
    defaults -currentHost write com.apple.controlcenter AirplayRecieverEnabled -bool false
    killall ControlCenter
    sleep 3
  Risk if you do not fix it
    Devices on the network can find the Mac and send content to it, and two more ports stay open.
  Risk when you fix it
    You can no longer AirPlay from an iPhone or iPad to this Mac. Control Center restarts briefly.
  Source
    Not in the guide; common hardening added by macguard
  Apply this fix? [y/N]
```

After a fix, macguard checks the item again and tells you if it still fails, together with the manual
steps and an offer to open the right pane of System Settings. Items that a script should not change
(FileVault, SIP, firmware, backups, the admin account) are always manual.

## Checks

The full reference, with the exact command, effect and both risks for every item, is generated from
the code: **[docs/checks.md](docs/checks.md)** ([Tiếng Việt](docs/checks.vi.md)).

| Priority | Items |
|---|---|
| High | `filevault`, `sip`, `gatekeeper`, `firmware`, `updates`, `autologin`, `firewall` |
| Medium | `pending_updates`, `xprotect`, `backup`, `screenlock`, `guest`, `stealth`, `certs`, `ssh`, `ssh_password`, `screensharing`, `airplay`, `listeners` |
| Low | `hint`, `admin_user`, `hostname`, `sudo_home`, `signed`, `dns`, `wifi`, `bonjour`, `handoff`, `brew_analytics`, `brew_redirect`, `diagnostics`, `crash_dialog`, `icloud_save`, `extensions`, `hidden_files`, `umask`, `terminal_keyboard` |
| For review (not scored) | `lockdown`, `outbound_fw`, `hosts_blocklist`, `bluetooth`, `launch_items`, `sysext`, `app_runtime` |

Parts of the guide that are advice rather than a setting (threat modelling, passwords and MFA, VPN,
browsers, Tor, messengers, pf rules, monitoring, physical access) are summarised by `macguard tips`.

## How closely it follows the guide

- **From the guide (40 items):** every check except the four below maps to a recommendation or
  command in the guide.
- **Added by macguard (4 items):** `autologin`, `guest`, `screensharing` and `airplay` are not in the
  guide. They are widely recommended baseline settings (CIS benchmarks, Apple's own guidance) and are
  marked "added by macguard" in the analysis and in [docs/checks.md](docs/checks.md).
- **Different method, same goal:** the guide sets the screen-saver password with
  `defaults write com.apple.screensaver askForPassword`; current macOS no longer reads those keys, so
  `screenlock` uses `sysadminctl -screenLock`, the same setting as System Settings.
- **Stricter reading:** the guide says to keep macOS up to date; `updates` also requires automatic
  installation of macOS updates.
- **Not automated:** advice that depends on your choices or accounts (Advanced Data Protection, a
  password manager, MFA, VPN, browser setup, Private Wi-Fi Address, pf, Privoxy) is in `macguard tips`
  instead of being scored.

## Safety and privacy

- `check`, `explain`, `list`, `tips` and the menu only read settings until you press `f` and answer `y`.
- Without an interactive terminal every confirmation is "no", so piping or scripting macguard cannot
  apply a fix by accident.
- System-wide fixes use `sudo` and prompt for your password. Per-user fixes run as you, even when
  macguard runs under sudo.
- Fixes that write files use separate, validated drop-in files that are easy to undo:
  `/etc/sudoers.d/macguard-env-home` (checked with `visudo -c`) and
  `/etc/ssh/sshd_config.d/000-macguard.conf` (checked with `sshd -t`, removed if invalid).
- macguard collects nothing and sends nothing. The only network access is `macguard update`, which
  contacts GitHub when you run it.
- Hardening always trades some convenience for security. Read the "risk when you fix it" line, and
  ignore what does not fit your threat model, such as Bluetooth or Screen Sharing on a Mac without a
  keyboard or display.

## Development

```bash
cargo test                          # unit and command-line tests (read-only, never applies a fix)
python3 -I tests/menu_pty.py        # drives the menu in a pseudo-terminal (build with --release first)
cargo run -- gen-docs docs          # regenerate docs/checks.md and docs/checks.vi.md
cargo fmt && cargo clippy --all-targets -- -D warnings
```

- `data/checks.toml` holds every item: severity, area, the texts in both languages and the fix
  script. `extra = true` marks items that are not in the guide.
- `src/checks.rs` holds the check logic, one `match` arm per id. A test fails if an id is missing on
  either side.
- `data/tips.toml` holds the advice shown by `macguard tips`.

To publish a release, set `version` in `Cargo.toml`, then push a tag `vX.Y.Z`. The release workflow
builds the universal binary and its checksum, which `macguard update` installs.

Pull requests are welcome. Please keep both languages complete and regenerate the docs.

## Credits and license

Recommendations come from [drduh/macOS-Security-and-Privacy-Guide](https://github.com/drduh/macOS-Security-and-Privacy-Guide).
The menu is inspired by [Mole](https://github.com/tw93/Mole). macguard is an independent project and
is not affiliated with the guide's authors, with Mole or with Apple.

MIT License. See [LICENSE](LICENSE).
