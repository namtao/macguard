# macguard

[English](README.md) · **Tiếng Việt**

Công cụ dòng lệnh chạy nhanh, kiểm tra và gia cố bảo mật macOS dựa theo tài liệu
[macOS Security and Privacy Guide](https://github.com/drduh/macOS-Security-and-Privacy-Guide) của drduh.

- **55 mục kiểm**: 42 mục lấy từ các khuyến nghị kiểm được trong tài liệu (mã hoá ổ đĩa, bảo mật khởi
  động, cập nhật, XProtect, sao lưu, đăng nhập, tường lửa, DNS, chứng chỉ, Wi-Fi, máy chủ và khoá SSH,
  cổng đang mở, tiến trình nền, system extension, Homebrew và mặc định về quyền riêng tư), 13 mục là
  bước gia cố phổ biến mà tài liệu không nhắc (các dịch vụ chia sẻ, sudo không mật khẩu, độ mạnh khoá
  SSH và chuyển tiếp agent, cùng vài cài đặt đăng nhập). Xem [Mức độ bám sát tài liệu](#mức-độ-bám-sát-tài-liệu).
- **Menu tương tác** giống lệnh `mo` của [Mole](https://github.com/tw93/Mole), xếp theo mức ưu tiên:
  cao, rồi vừa, rồi thấp, mục chưa đạt lên trước. Mở phân tích, sửa hoặc bỏ qua chỉ bằng một phím. Chỉ
  thoát khi bấm `q` hoặc Ctrl-C.
- **Mỗi bản sửa đều được giải thích trước khi chạy**: sửa gì, lệnh chính xác, tác dụng, rủi ro nếu
  không sửa và rủi ro khi sửa. Không có gì chạy nếu bạn không bấm `y`.
- **Bỏ qua mục không hợp với bạn.** Mục đã bỏ qua sẽ không bị kiểm ở lần sau.
- **Tiếng Anh và tiếng Việt**, tự chọn theo ngôn ngữ macOS hoặc dùng `--lang en|vi`.
- Một file chạy duy nhất viết bằng Rust. Mọi mục kiểm chạy song song (khoảng một giây), menu phản hồi
  tức thì, và lần duy nhất kết nối mạng là khi bạn chạy `macguard update`.

## Yêu cầu

macOS 13 Ventura trở lên, chip Apple hoặc Intel. Một số mục cần `sudo` và sẽ được bỏ qua nếu không có.

## Cài đặt

Tải bản phát hành mới nhất (một file chạy chung cho chip Apple và Intel):

```bash
curl -fsSLO https://github.com/namtao/macguard/releases/latest/download/macguard-macos-universal.tar.gz
curl -fsSLO https://github.com/namtao/macguard/releases/latest/download/macguard-macos-universal.tar.gz.sha256
shasum -a 256 -c macguard-macos-universal.tar.gz.sha256
tar -xzf macguard-macos-universal.tar.gz
sudo install -m 755 macguard /usr/local/bin/macguard
```

Hoặc build từ mã nguồn với Rust 1.80 trở lên:

```bash
cargo install --git https://github.com/namtao/macguard
```

### Cập nhật

```bash
macguard update          # dùng 'sudo macguard update' nếu file nằm ở thư mục bạn không ghi được
```

`macguard update` hỏi GitHub bản phát hành mới nhất, tải bằng `curl`, kiểm tra checksum SHA-256 rồi
thay file đang cài trong một bước nguyên tử. Nếu bạn cài bằng `cargo install`, chạy lại
`cargo install --git https://github.com/namtao/macguard --force` cũng được.

### Gỡ cài đặt

Xoá file chạy (`sudo rm /usr/local/bin/macguard`, hoặc `cargo uninstall macguard`) và, nếu đã dùng danh
sách bỏ qua, xoá `~/.config/macguard`.

## Cách dùng

### Menu tương tác

```bash
macguard          # mở menu
sudo macguard     # kiểm cả mục cần root (firmware, sudoers, mọi cổng đang mở)
```

Menu hiện ngay và điền dần kết quả khi từng mục kiểm xong. Các mục được nhóm theo mức ưu tiên:

```text
macguard 1.0.0 · macOS 15.5
Điểm 28/34 · 2 mục cao/vừa chưa đạt
──────────────────────────────────────────────────────────────────────────────
 Mức cao
   ✓ filevault          Mã hoá ổ đĩa bằng FileVault
   ✓ firewall           Bật tường lửa ứng dụng
 Mức vừa · 2 chưa đạt
   ✗ backup             Sao lưu Time Machine có mã hoá (chưa có ổ sao lưu)
 ▸ ✗ screenlock         Đòi mật khẩu ngay khi ngủ hoặc bật màn hình chờ (chờ 300 giây)
   ✓ ssh                Tắt Đăng nhập từ xa (máy chủ SSH)
 Mức thấp · 3 chưa đạt
   ! dns                DNS mã hoá hoặc có lọc
   ⊘ bluetooth          Tắt Bluetooth khi không dùng
──────────────────────────────────────────────────────────────────────────────
↑↓ chọn · Enter phân tích · f sửa · i bỏ qua/bỏ bỏ qua · a chỉ mục chưa đạt/tất cả · r quét lại · q thoát
```

| Phím | Tác dụng |
|---|---|
| `↑` `↓` (hoặc `k` `j`), PgUp, PgDn, Home, End | Di chuyển |
| Enter (hoặc `→`) | Mở phân tích: tác dụng, lệnh chính xác, rủi ro khi sửa và khi không sửa, nguồn |
| `f` | Sửa mục đang chọn. macguard hiện phân tích và hỏi `y/N` trước khi chạy bất cứ lệnh gì |
| `i` | Bỏ qua mục (lần sau không kiểm), hoặc kiểm lại mục đã bỏ qua |
| `o` (trong màn phân tích) | Mở đúng trang trong Cài đặt hệ thống |
| `a` | Chỉ hiện mục chưa đạt, hoặc hiện tất cả |
| `r` | Quét lại |
| `←` hoặc Esc (trong màn phân tích) | Quay lại danh sách |
| `q` hoặc Ctrl-C | Thoát |

### Lệnh dùng trong script

```bash
macguard check              # báo cáo theo mức; mã thoát 1 khi còn mục cao hoặc vừa chưa đạt
macguard explain screenlock # phân tích một hoặc nhiều mục
macguard fix                # đi qua từng mục chưa đạt và hỏi trước mỗi lần sửa
macguard fix airplay ssh    # chỉ sửa các mục này
macguard ignore bluetooth   # không kiểm mục này nữa
macguard unignore bluetooth # kiểm lại mục này
macguard list               # mã các mục theo mức (⊘ = đang bỏ qua)
macguard tips               # lời khuyên trong tài liệu không tự kiểm được
macguard update             # cài bản phát hành mới nhất
macguard --lang vi check    # kết quả tiếng Việt (hoặc: export MACGUARD_LANG=vi)
```

Khi không có terminal (ví dụ trong pipe hay cron), gõ `macguard` không kèm lệnh sẽ in báo cáo thay vì
mở menu. Danh sách bỏ qua được lưu theo từng người dùng ở `~/.config/macguard/ignored`, mỗi dòng một
mã.

Ký hiệu: `✓` đạt, `✗` đỏ mức cao, `✗` vàng mức vừa, `!` mức thấp, `i` tham khảo, `-` không áp dụng
(kèm lý do), `⊘` đang bỏ qua, `…` đang kiểm.

### Màn hình hiện gì trước khi sửa

```text
✗ Tắt Bộ thu AirPlay (airplay, vừa, Chia sẻ và cổng mở)
  Trạng thái
    chưa đạt
  Tác dụng
    Trung tâm điều khiển ngừng mở cổng 5000 và 7000 trên mọi giao diện mạng.
  Lệnh sẽ chạy
    defaults -currentHost write com.apple.controlcenter AirplayRecieverEnabled -bool false
    killall ControlCenter
    sleep 3
  Rủi ro nếu không sửa
    Thiết bị trong mạng dò thấy máy và gửi nội dung tới máy được, đồng thời thêm hai cổng luôn mở.
  Rủi ro khi sửa
    Bạn không còn AirPlay từ iPhone hay iPad sang máy này được. Trung tâm điều khiển khởi động lại
    trong giây lát.
  Nguồn
    Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến
  Sửa mục này? [y/N]
```

Sau mỗi lần sửa, macguard kiểm lại mục đó; nếu vẫn chưa đạt, nó in cách làm tay và đề nghị mở đúng mục
trong Cài đặt hệ thống. Những mục script không nên tự đổi (FileVault, SIP, firmware, sao lưu, tài khoản
quản trị) luôn phải làm tay.

## Các mục kiểm

Tài liệu tham chiếu đầy đủ, gồm lệnh chính xác, tác dụng và rủi ro hai chiều của từng mục, được sinh tự
động từ code: **[docs/checks.vi.md](docs/checks.vi.md)** ([English](docs/checks.md)).

| Mức | Mục |
|---|---|
| Cao | `filevault`, `sip`, `gatekeeper`, `firmware`, `updates`, `autologin`, `firewall` |
| Vừa | `pending_updates`, `xprotect`, `backup`, `screenlock`, `guest`, `sudo_password`, `ssh_key_passphrase`, `ssh_host_keys`, `stealth`, `certs`, `ssh`, `ssh_password`, `screensharing`, `airplay`, `listeners`, `file_sharing`, `internet_sharing`, `remote_management`, `remote_apple_events` |
| Thấp | `hint`, `admin_user`, `hostname`, `sudo_home`, `ssh_key_strength`, `ssh_agent_forwarding`, `signed`, `dns`, `wifi`, `bonjour`, `handoff`, `printer_sharing`, `media_sharing`, `brew_analytics`, `brew_redirect`, `diagnostics`, `crash_dialog`, `icloud_save`, `extensions`, `hidden_files`, `umask`, `terminal_keyboard` |
| Tham khảo (không tính điểm) | `lockdown`, `outbound_fw`, `hosts_blocklist`, `bluetooth`, `launch_items`, `sysext`, `app_runtime` |

Những phần của tài liệu là lời khuyên chứ không phải một cài đặt (mô hình mối đe doạ, mật khẩu và xác
thực nhiều lớp, VPN, trình duyệt, Tor, nhắn tin, luật pf, giám sát, truy cập vật lý) được tóm tắt trong
`macguard tips`.

## Mức độ bám sát tài liệu

- **Lấy từ tài liệu (42 mục):** mọi mục, trừ các mục dưới đây, đều ứng với một khuyến nghị hoặc lệnh
  trong tài liệu, kể cả lời khuyên về SSH: khoá có passphrase hoặc nằm trên khoá phần cứng, và có kiểm
  host key của máy chủ.
- **macguard thêm vào (13 mục):** `autologin`, `guest`, `sudo_password`, `ssh_key_strength`,
  `ssh_agent_forwarding`, `screensharing`, `airplay`, `file_sharing`, `printer_sharing`,
  `internet_sharing`, `remote_management`, `remote_apple_events` và `media_sharing` không có trong tài
  liệu. Đây là các cài đặt nền tảng được khuyên dùng rộng rãi (bộ chuẩn CIS, hướng dẫn của Apple) và được
  ghi "macguard thêm" trong màn phân tích và trong [docs/checks.vi.md](docs/checks.vi.md).
- **Khác cách làm, cùng mục tiêu:** tài liệu đặt mật khẩu màn hình chờ bằng
  `defaults write com.apple.screensaver askForPassword`; macOS hiện tại không còn đọc các khoá đó, nên
  `screenlock` dùng `sysadminctl -screenLock`, đúng tuỳ chọn trong Cài đặt hệ thống.
- **Hiểu chặt hơn:** tài liệu yêu cầu luôn cập nhật macOS; mục `updates` đòi thêm việc tự cài bản macOS.
- **Không tự động hoá:** lời khuyên phụ thuộc lựa chọn hay tài khoản của bạn (Bảo vệ dữ liệu nâng cao,
  trình quản lý mật khẩu, xác thực nhiều lớp, VPN, cài đặt trình duyệt, Địa chỉ Wi-Fi riêng, pf, Privoxy)
  nằm trong `macguard tips` thay vì được chấm điểm.

## An toàn và quyền riêng tư

- `check`, `explain`, `list`, `tips` và menu chỉ đọc cài đặt, cho tới khi bạn bấm `f` và trả lời `y`.
  Dịch vụ chia sẻ được phát hiện bằng cách thử kết nối tới cổng của nó ngay trên máy (127.0.0.1); khoá
  SSH được kiểm bằng `ssh-keygen` và không bao giờ rời khỏi máy.
- Khi không có terminal tương tác, mọi câu hỏi xác nhận đều là "không", nên chạy macguard qua pipe hay
  script không thể vô tình áp dụng bản sửa nào.
- Bản sửa toàn hệ thống dùng `sudo` và sẽ hỏi mật khẩu. Bản sửa theo người dùng chạy dưới tên bạn, kể
  cả khi macguard chạy bằng sudo.
- Bản sửa ghi file đều dùng file riêng, có kiểm tra và dễ hoàn tác:
  `/etc/sudoers.d/macguard-env-home` (kiểm bằng `visudo -c`) và
  `/etc/ssh/sshd_config.d/000-macguard.conf` (kiểm bằng `sshd -t`, tự xoá nếu sai).
- macguard không thu thập và không gửi gì đi. Lần duy nhất kết nối mạng là `macguard update`, chỉ khi
  bạn chạy lệnh đó.
- Gia cố bảo mật luôn đổi một phần tiện lợi lấy an toàn. Hãy đọc dòng "rủi ro khi sửa" và bỏ qua mục
  không hợp mô hình mối đe doạ của bạn, ví dụ Bluetooth hay Chia sẻ màn hình trên máy không gắn bàn phím,
  màn hình.

## Phát triển

```bash
cargo test                          # test đơn vị và test dòng lệnh (chỉ đọc, không bao giờ sửa)
python3 -I tests/menu_pty.py        # điều khiển menu trong terminal giả lập (build --release trước)
cargo run -- gen-docs docs          # sinh lại docs/checks.md và docs/checks.vi.md
cargo fmt && cargo clippy --all-targets -- -D warnings
```

- `data/checks.toml` chứa mọi mục: mức, nhóm, nội dung hai thứ tiếng và lệnh sửa. `extra = true` đánh
  dấu mục không có trong tài liệu.
- `src/checks.rs` chứa logic kiểm, mỗi mã một nhánh `match`. Test sẽ báo lỗi nếu một mã thiếu ở một
  trong hai bên.
- `data/tips.toml` chứa lời khuyên của `macguard tips`.

Để phát hành bản mới, sửa `version` trong `Cargo.toml` rồi đẩy tag `vX.Y.Z`. Workflow phát hành sẽ build
file chạy chung và checksum, là thứ `macguard update` tải về.

Rất hoan nghênh pull request. Hãy giữ đủ cả hai thứ tiếng và sinh lại docs.

## Nguồn và giấy phép

Các khuyến nghị lấy từ [drduh/macOS-Security-and-Privacy-Guide](https://github.com/drduh/macOS-Security-and-Privacy-Guide).
Menu lấy cảm hứng từ [Mole](https://github.com/tw93/Mole). macguard là dự án độc lập, không liên kết với
tác giả tài liệu, với Mole hay với Apple.

Giấy phép MIT. Xem [LICENSE](LICENSE).
