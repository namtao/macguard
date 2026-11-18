# Các mục kiểm của macguard

Được sinh bởi `cargo run -- gen-docs docs` từ `data/checks.toml`. Đừng sửa tay.

Các mục được nhóm theo mức độ. Mục **cao** và **vừa** ảnh hưởng mã thoát của `macguard check`; mục **thấp** là gia cố nên làm; mục **tham khảo** không tính điểm.

## Mức cao

| Mã | Mục kiểm | Nhóm | Cách sửa | Nguồn |
|---|---|---|---|---|
| [`filevault`](#filevault) | Mã hoá ổ đĩa bằng FileVault | Hệ thống | làm tay | tài liệu |
| [`sip`](#sip) | Bảo vệ toàn vẹn hệ thống (SIP) | Hệ thống | làm tay | tài liệu |
| [`gatekeeper`](#gatekeeper) | Gatekeeper kiểm tra app tải về | Hệ thống | tự động, hỏi trước | tài liệu |
| [`firmware`](#firmware) | Bảo mật khởi động ở mức Full Security | Hệ thống | làm tay | tài liệu |
| [`updates`](#updates) | Tự động cập nhật bảo mật | Hệ thống | tự động, hỏi trước | tài liệu |
| [`autologin`](#autologin) | Tắt tự động đăng nhập | Tài khoản và đăng nhập | tự động, hỏi trước | macguard thêm |
| [`firewall`](#firewall) | Bật tường lửa ứng dụng | Mạng và tường lửa | tự động, hỏi trước | tài liệu |

## Mức vừa

| Mã | Mục kiểm | Nhóm | Cách sửa | Nguồn |
|---|---|---|---|---|
| [`pending_updates`](#pending_updates) | Không còn bản cập nhật chờ cài | Hệ thống | làm tay | tài liệu |
| [`xprotect`](#xprotect) | Dữ liệu nhận diện mã độc XProtect còn mới | Hệ thống | tự động, hỏi trước | tài liệu |
| [`backup`](#backup) | Sao lưu Time Machine có mã hoá | Hệ thống | làm tay | tài liệu |
| [`screenlock`](#screenlock) | Đòi mật khẩu ngay khi ngủ hoặc bật màn hình chờ | Tài khoản và đăng nhập | tự động, hỏi trước | tài liệu |
| [`guest`](#guest) | Tắt tài khoản Khách (Guest) | Tài khoản và đăng nhập | tự động, hỏi trước | macguard thêm |
| [`stealth`](#stealth) | Tường lửa chế độ ẩn (stealth) | Mạng và tường lửa | tự động, hỏi trước | tài liệu |
| [`certs`](#certs) | Không có chứng chỉ được tin cậy thêm | Mạng và tường lửa | làm tay | tài liệu |
| [`ssh`](#ssh) | Tắt Đăng nhập từ xa (máy chủ SSH) | Chia sẻ và cổng mở | tự động, hỏi trước | tài liệu |
| [`ssh_password`](#ssh_password) | Máy chủ SSH chỉ nhận khoá, không nhận mật khẩu | Chia sẻ và cổng mở | tự động, hỏi trước | tài liệu |
| [`screensharing`](#screensharing) | Tắt Chia sẻ màn hình | Chia sẻ và cổng mở | tự động, hỏi trước | macguard thêm |
| [`airplay`](#airplay) | Tắt Bộ thu AirPlay | Chia sẻ và cổng mở | tự động, hỏi trước | macguard thêm |
| [`listeners`](#listeners) | Không dịch vụ nào khác mở cổng ra mạng | Chia sẻ và cổng mở | làm tay | tài liệu |
| [`file_sharing`](#file_sharing) | Tắt Chia sẻ tệp | Chia sẻ và cổng mở | tự động, hỏi trước | macguard thêm |
| [`internet_sharing`](#internet_sharing) | Tắt Chia sẻ Internet | Chia sẻ và cổng mở | làm tay | macguard thêm |
| [`remote_management`](#remote_management) | Tắt Quản lý từ xa (Apple Remote Desktop) | Chia sẻ và cổng mở | tự động, hỏi trước | macguard thêm |
| [`remote_apple_events`](#remote_apple_events) | Tắt Điều khiển ứng dụng từ xa bằng script | Chia sẻ và cổng mở | tự động, hỏi trước | macguard thêm |
| [`sudo_password`](#sudo_password) | sudo luôn đòi mật khẩu | Tài khoản và đăng nhập | làm tay | macguard thêm |
| [`ssh_key_passphrase`](#ssh_key_passphrase) | Khoá SSH riêng được bảo vệ bằng passphrase | Tài khoản và đăng nhập | làm tay | tài liệu |
| [`ssh_host_keys`](#ssh_host_keys) | SSH có kiểm tra host key của máy chủ | Tài khoản và đăng nhập | làm tay | tài liệu |

## Mức thấp

| Mã | Mục kiểm | Nhóm | Cách sửa | Nguồn |
|---|---|---|---|---|
| [`hint`](#hint) | Không hiện gợi ý mật khẩu | Tài khoản và đăng nhập | tự động, hỏi trước | tài liệu |
| [`admin_user`](#admin_user) | Tài khoản dùng hằng ngày là tài khoản thường | Tài khoản và đăng nhập | làm tay | tài liệu |
| [`hostname`](#hostname) | Tên máy không lộ danh tính | Tài khoản và đăng nhập | tự động, hỏi trước | tài liệu |
| [`sudo_home`](#sudo_home) | sudo không giữ biến HOME của bạn | Tài khoản và đăng nhập | tự động, hỏi trước | tài liệu |
| [`signed`](#signed) | Tường lửa hỏi trước khi cho app đã ký nhận kết nối | Mạng và tường lửa | tự động, hỏi trước | tài liệu |
| [`dns`](#dns) | DNS mã hoá hoặc có lọc | Mạng và tường lửa | làm tay | tài liệu |
| [`wifi`](#wifi) | Wi-Fi dùng WPA3 | Mạng và tường lửa | làm tay | tài liệu |
| [`bonjour`](#bonjour) | Tắt quảng bá Bonjour | Mạng và tường lửa | tự động, hỏi trước | tài liệu |
| [`handoff`](#handoff) | Tắt Handoff | Mạng và tường lửa | tự động, hỏi trước | tài liệu |
| [`brew_analytics`](#brew_analytics) | Tắt thống kê của Homebrew | Phần mềm | tự động, hỏi trước | tài liệu |
| [`brew_redirect`](#brew_redirect) | Homebrew từ chối chuyển hướng HTTPS sang HTTP | Phần mềm | tự động, hỏi trước | tài liệu |
| [`diagnostics`](#diagnostics) | Không chia sẻ dữ liệu chẩn đoán và sử dụng | Quyền riêng tư | tự động, hỏi trước | tài liệu |
| [`crash_dialog`](#crash_dialog) | Tắt hộp thoại gửi báo cáo lỗi | Quyền riêng tư | tự động, hỏi trước | tài liệu |
| [`icloud_save`](#icloud_save) | Tài liệu mới mặc định lưu trên máy | Quyền riêng tư | tự động, hỏi trước | tài liệu |
| [`extensions`](#extensions) | Finder hiện mọi phần mở rộng của tên file | Quyền riêng tư | tự động, hỏi trước | tài liệu |
| [`hidden_files`](#hidden_files) | Finder hiện file ẩn và thư mục ~/Library | Quyền riêng tư | tự động, hỏi trước | tài liệu |
| [`umask`](#umask) | File mới mặc định chỉ mình bạn đọc được (umask 077) | Quyền riêng tư | tự động, hỏi trước | tài liệu |
| [`terminal_keyboard`](#terminal_keyboard) | Bật Nhập bàn phím an toàn trong Terminal | Quyền riêng tư | tự động, hỏi trước | tài liệu |
| [`printer_sharing`](#printer_sharing) | Tắt Chia sẻ máy in và quản trị máy in từ xa | Chia sẻ và cổng mở | tự động, hỏi trước | macguard thêm |
| [`media_sharing`](#media_sharing) | Tắt Chia sẻ phương tiện | Chia sẻ và cổng mở | làm tay | macguard thêm |
| [`ssh_key_strength`](#ssh_key_strength) | Khoá SSH dùng thuật toán đủ mạnh | Tài khoản và đăng nhập | làm tay | macguard thêm |
| [`ssh_agent_forwarding`](#ssh_agent_forwarding) | Tắt chuyển tiếp SSH agent | Tài khoản và đăng nhập | làm tay | macguard thêm |

## Tham khảo (không tính điểm)

| Mã | Mục kiểm | Nhóm | Cách sửa | Nguồn |
|---|---|---|---|---|
| [`lockdown`](#lockdown) | Chế độ phong toả (Lockdown Mode) | Hệ thống | làm tay | tài liệu |
| [`outbound_fw`](#outbound_fw) | Có app tường lửa kiểm soát kết nối đi ra | Mạng và tường lửa | làm tay | tài liệu |
| [`hosts_blocklist`](#hosts_blocklist) | File hosts chặn tên miền xấu đã biết | Mạng và tường lửa | làm tay | tài liệu |
| [`bluetooth`](#bluetooth) | Tắt Bluetooth khi không dùng | Mạng và tường lửa | làm tay | tài liệu |
| [`launch_items`](#launch_items) | Đã rà soát tiến trình nền của bên thứ ba | Phần mềm | làm tay | tài liệu |
| [`sysext`](#sysext) | Đã rà soát system extension | Phần mềm | làm tay | tài liệu |
| [`app_runtime`](#app_runtime) | App dùng Hardened Runtime | Phần mềm | làm tay | tài liệu |

## Chi tiết

<a id="filevault"></a>

### `filevault`: Mã hoá ổ đĩa bằng FileVault

- **Mức:** cao
- **Nhóm:** Hệ thống
- **Tác dụng:** Bắt buộc nhập mật khẩu để mở khoá ổ khởi động sau mỗi lần khởi động lại. Trên chip Apple, FileVault còn bảo vệ firmware: không ai khởi động được từ ổ khác, vào Recovery hay hồi sinh máy bằng DFU nếu không được phép.
- **Cách sửa (làm tay):** Cài đặt hệ thống > Quyền riêng tư & Bảo mật > FileVault > Bật, rồi cất khoá khôi phục ở nơi an toàn.
- **Rủi ro nếu không sửa:** Người lấy được máy có thể đọc dữ liệu của bạn sau khi khởi động lại mà không cần biết mật khẩu.
- **Rủi ro khi sửa:** Nếu quên mật khẩu đăng nhập và mất khoá khôi phục, dữ liệu mất vĩnh viễn. Hãy cất khoá khôi phục ở nơi an toàn.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="sip"></a>

### `sip`: Bảo vệ toàn vẹn hệ thống (SIP)

- **Mức:** cao
- **Nhóm:** Hệ thống
- **Tác dụng:** Ngăn mọi tiến trình, kể cả tiến trình chạy quyền root, sửa file, dịch vụ và tiến trình được hệ thống bảo vệ.
- **Cách sửa (làm tay):** Chỉ bật được trong Recovery: tắt máy, giữ nút nguồn, chọn Options, mở Utilities > Terminal, chạy 'csrutil enable' rồi khởi động lại.
- **Rủi ro nếu không sửa:** Mã độc có quyền quản trị có thể sửa chính macOS và ẩn mình trong dịch vụ hệ thống.
- **Rủi ro khi sửa:** Công cụ cần sửa hoặc theo dõi hệ thống (ví dụ DTrace) sẽ không chạy được.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="gatekeeper"></a>

### `gatekeeper`: Gatekeeper kiểm tra app tải về

- **Mức:** cao
- **Nhóm:** Hệ thống
- **Tác dụng:** Kiểm tra chữ ký và chứng nhận (notarization) của Apple trước khi app được mở lần đầu.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo spctl --global-enable
```

- **Nếu sửa tự động không được:** Chạy 'sudo spctl --global-enable', hoặc chọn cho phép app từ App Store và nhà phát triển đã xác định trong Cài đặt > Quyền riêng tư & Bảo mật.
- **Rủi ro nếu không sửa:** App không chữ ký hoặc bị sửa, con đường phát tán mã độc phổ biến, sẽ mở mà không có cảnh báo nào.
- **Rủi ro khi sửa:** Bạn sẽ bị cảnh báo khi mở app chưa được chứng nhận. Vẫn có thể cho phép từng app trong Quyền riêng tư & Bảo mật.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="firmware"></a>

### `firmware`: Bảo mật khởi động ở mức Full Security

- **Mức:** cao
- **Nhóm:** Hệ thống
- **Tác dụng:** Chỉ cho khởi động macOS bản hiện hành có chữ ký của Apple. Đây là mặc định.
- **Cách sửa (làm tay):** Vào Recovery, mở Utilities > Startup Security Utility, chọn ổ khởi động rồi chọn Full Security.
- **Rủi ro nếu không sửa:** Hệ điều hành cũ có lỗ hổng, bản bị sửa và kernel extension bên thứ ba có thể được nạp.
- **Rủi ro khi sửa:** Kernel extension bên thứ ba và các bản macOS cũ sẽ không nạp hoặc không khởi động được nữa.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="updates"></a>

### `updates`: Tự động cập nhật bảo mật

- **Mức:** cao
- **Nhóm:** Hệ thống
- **Tác dụng:** Bật tự kiểm tra, tự tải, tự cài bản vá bảo mật khẩn, dữ liệu hệ thống (XProtect) và bản cập nhật macOS. Tài liệu yêu cầu luôn cập nhật hệ thống; việc bắt buộc tự cài bản macOS là cách hiểu chặt hơn của macguard.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
for key in AutomaticCheckEnabled AutomaticDownload CriticalUpdateInstall ConfigDataInstall AutomaticallyInstallMacOSUpdates; do
  sudo defaults write /Library/Preferences/com.apple.SoftwareUpdate "$key" -bool true
done
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Cài đặt chung > Cập nhật phần mềm > Cập nhật tự động: bật tất cả.
- **Rủi ro nếu không sửa:** Máy tiếp tục dính các lỗ hổng đã công bố, kể cả lỗ hổng đang bị khai thác ngoài thực tế.
- **Rủi ro khi sửa:** macOS có thể tự khởi động lại để cài xong bản cập nhật, nên hãy lưu công việc. Hiếm khi một bản cập nhật làm hỏng app.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="autologin"></a>

### `autologin`: Tắt tự động đăng nhập

- **Mức:** cao
- **Nhóm:** Tài khoản và đăng nhập
- **Tác dụng:** Bắt buộc nhập mật khẩu ở màn hình đăng nhập sau mỗi lần khởi động lại.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo sysadminctl -autologin off
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Người dùng & Nhóm > Tự động đăng nhập bằng: Tắt.
- **Rủi ro nếu không sửa:** Ai bật máy cũng vào thẳng tài khoản của bạn. Tự đăng nhập còn lưu mật khẩu trong một file được bảo vệ kém (/etc/kcpassword).
- **Rủi ro khi sửa:** Bạn phải gõ mật khẩu sau mỗi lần khởi động lại. Máy chạy không người trông sẽ dừng ở màn hình đăng nhập sau khi mất điện.
- **Nguồn:** Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến

<a id="firewall"></a>

### `firewall`: Bật tường lửa ứng dụng

- **Mức:** cao
- **Nhóm:** Mạng và tường lửa
- **Tác dụng:** Kiểm soát app nào được nhận kết nối mạng từ bên ngoài. Không lọc kết nối đi ra.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate on
sudo pkill -HUP socketfilterfw
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Mạng > Tường lửa: Bật.
- **Rủi ro nếu không sửa:** Mọi app hay dịch vụ đang mở cổng đều bị thiết bị cùng mạng truy cập được.
- **Rủi ro khi sửa:** macOS sẽ hỏi lần đầu khi một app mới muốn nhận kết nối. Mã độc có quyền quản trị vẫn tắt được tường lửa.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="pending_updates"></a>

### `pending_updates`: Không còn bản cập nhật chờ cài

- **Mức:** vừa
- **Nhóm:** Hệ thống
- **Tác dụng:** Xác nhận các bản cập nhật macOS đã tìm thấy đều đã được cài. Dùng kết quả lần quét nền gần nhất, không kết nối tới Apple.
- **Cách sửa (làm tay):** Cài đặt hệ thống > Cài đặt chung > Cập nhật phần mềm > Cập nhật ngay, hoặc chạy 'softwareupdate --list' rồi 'sudo softwareupdate --install --all'.
- **Rủi ro nếu không sửa:** Các bản vá bảo mật đã có sẵn vẫn chưa được cài.
- **Rủi ro khi sửa:** Cài đặt có thể cần khởi động lại và hiếm khi làm hỏng một app.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="xprotect"></a>

### `xprotect`: Dữ liệu nhận diện mã độc XProtect còn mới

- **Mức:** vừa
- **Nhóm:** Hệ thống
- **Tác dụng:** XProtect là phần mềm diệt virus có sẵn trong macOS, nhận diện và gỡ mã độc đã biết bằng dữ liệu Apple cập nhật ngầm.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo xprotect update
```

- **Nếu sửa tự động không được:** Chạy 'sudo xprotect update' và giữ bật tự động cập nhật bảo mật.
- **Rủi ro nếu không sửa:** Các dòng mã độc mới ra từ lần cập nhật trước sẽ không bị phát hiện.
- **Rủi ro khi sửa:** Không có rủi ro đáng kể. Lệnh tải dữ liệu từ Apple.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="backup"></a>

### `backup`: Sao lưu Time Machine có mã hoá

- **Mức:** vừa
- **Nhóm:** Hệ thống
- **Tác dụng:** Giữ bản sao khôi phục được, có mã hoá để ổ sao lưu bị mất hay bị lấy cũng không lộ dữ liệu. Tài liệu khuyên mô hình 3-2-1: ba bản sao, hai loại thiết bị, một bản để nơi khác.
- **Cách sửa (làm tay):** Cài đặt hệ thống > Cài đặt chung > Time Machine > Thêm ổ sao lưu, và đánh dấu Mã hoá bản sao lưu.
- **Rủi ro nếu không sửa:** Mã độc tống tiền, hỏng ổ hoặc mất máy có thể làm mất dữ liệu vĩnh viễn. Ổ sao lưu không mã hoá thì lộ toàn bộ dữ liệu trên đó.
- **Rủi ro khi sửa:** Mất mật khẩu sao lưu thì không đọc được bản sao lưu mã hoá. Nếu bạn sao lưu bằng công cụ khác, có thể bỏ qua mục này.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="screenlock"></a>

### `screenlock`: Đòi mật khẩu ngay khi ngủ hoặc bật màn hình chờ

- **Mức:** vừa
- **Nhóm:** Tài khoản và đăng nhập
- **Tác dụng:** Khoá phiên làm việc ngay khi màn hình chờ chạy hoặc màn hình tắt. Tài liệu dùng 'defaults write com.apple.screensaver askForPassword'; macOS hiện tại không còn đọc các khoá đó, nên macguard dùng sysadminctl, đặt đúng tuỳ chọn như trong Cài đặt hệ thống.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sysadminctl -screenLock immediate -password -
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Màn hình khoá > Yêu cầu mật khẩu sau khi bắt đầu trình bảo vệ màn hình hoặc tắt màn hình: Ngay lập tức.
- **Rủi ro nếu không sửa:** Trong khoảng chờ đó, người bên cạnh có thể dùng phiên đang mở của bạn.
- **Rủi ro khi sửa:** Bạn phải nhập mật khẩu (hoặc Touch ID) mỗi khi đánh thức màn hình. Lệnh sẽ hỏi mật khẩu đăng nhập của bạn.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="guest"></a>

### `guest`: Tắt tài khoản Khách (Guest)

- **Mức:** vừa
- **Nhóm:** Tài khoản và đăng nhập
- **Tác dụng:** Bỏ tài khoản Khách không mật khẩu khỏi màn hình đăng nhập.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo sysadminctl -guestAccount off
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Người dùng & Nhóm > Người dùng khách > Cho phép khách đăng nhập: Tắt.
- **Rủi ro nếu không sửa:** Ai cũng đăng nhập được không cần mật khẩu và dùng máy, mạng, thư mục chia sẻ.
- **Rủi ro khi sửa:** Khách muốn dùng máy phải có tài khoản riêng. Tính năng Tìm không còn định vị được máy bị mất qua người dùng tài khoản Khách.
- **Nguồn:** Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến

<a id="stealth"></a>

### `stealth`: Tường lửa chế độ ẩn (stealth)

- **Mức:** vừa
- **Nhóm:** Mạng và tường lửa
- **Tác dụng:** Máy không trả lời các lần dò, ping và kết nối tới cổng đang đóng.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setstealthmode on
sudo pkill -HUP socketfilterfw
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Mạng > Tường lửa > Tuỳ chọn > Bật chế độ ẩn.
- **Rủi ro nếu không sửa:** Công cụ quét mạng dễ dàng tìm thấy máy và chọn làm mục tiêu.
- **Rủi ro khi sửa:** Thiết bị khác không ping được máy, có thể gây khó khi dò lỗi mạng.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="certs"></a>

### `certs`: Không có chứng chỉ được tin cậy thêm

- **Mức:** vừa
- **Nhóm:** Mạng và tường lửa
- **Tác dụng:** Tìm chứng chỉ được người dùng, quản trị, VPN, phần mềm bảo mật hoặc MDM đánh dấu tin cậy ngoài bộ chứng chỉ gốc của Apple.
- **Cách sửa (làm tay):** Mở Keychain Access, xem các keychain login, Local Items và System, đặt chứng chỉ bạn không nhận ra thành Never Trust.
- **Rủi ro nếu không sửa:** Một chứng chỉ được tin cậy ngoài ý muốn cho phép người giữ khoá của nó âm thầm đọc trộm lưu lượng HTTPS của bạn.
- **Rủi ro khi sửa:** Bỏ tin cậy một chứng chỉ bạn thật sự cần (mạng công ty, VPN, proxy gỡ lỗi) sẽ làm hỏng các kết nối đó.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="ssh"></a>

### `ssh`: Tắt Đăng nhập từ xa (máy chủ SSH)

- **Mức:** vừa
- **Nhóm:** Chia sẻ và cổng mở
- **Tác dụng:** Tắt máy chủ SSH, vốn tắt sẵn trên macOS.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo systemsetup -f -setremotelogin off || {
  sudo launchctl disable system/com.openssh.sshd
  sudo launchctl bootout system/com.openssh.sshd
}
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Cài đặt chung > Chia sẻ > Đăng nhập từ xa: Tắt.
- **Rủi ro nếu không sửa:** Ai tới được cổng 22 cũng có thể thử đăng nhập, ví dụ dò mật khẩu.
- **Rủi ro khi sửa:** Bạn không còn kết nối vào máy bằng ssh, scp hay rsync qua SSH. Nếu cần SSH, hãy giữ bật và làm đạt mục ssh_password.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="ssh_password"></a>

### `ssh_password`: Máy chủ SSH chỉ nhận khoá, không nhận mật khẩu

- **Mức:** vừa
- **Nhóm:** Chia sẻ và cổng mở
- **Tác dụng:** Khi Đăng nhập từ xa đang bật, từ chối đăng nhập bằng mật khẩu và keyboard-interactive để chỉ khoá SSH dùng được. Ghi vào file riêng và kiểm bằng 'sshd -t'.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
printf 'PasswordAuthentication no\nKbdInteractiveAuthentication no\n' | sudo tee /etc/ssh/sshd_config.d/000-macguard.conf >/dev/null
sudo sshd -t || sudo rm -f /etc/ssh/sshd_config.d/000-macguard.conf
```

- **Nếu sửa tự động không được:** Thêm 'PasswordAuthentication no' và 'KbdInteractiveAuthentication no' vào một file trong /etc/ssh/sshd_config.d/, rồi chạy 'sudo sshd -t'.
- **Rủi ro nếu không sửa:** Kẻ tấn công tới được cổng 22 có thể đoán hoặc dùng lại mật khẩu của bạn để vào máy.
- **Rủi ro khi sửa:** Nếu khoá công khai của bạn chưa có trong ~/.ssh/authorized_keys, bạn sẽ tự khoá mình khỏi SSH. Hoàn tác bằng cách xoá /etc/ssh/sshd_config.d/000-macguard.conf.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="screensharing"></a>

### `screensharing`: Tắt Chia sẻ màn hình

- **Mức:** vừa
- **Nhóm:** Chia sẻ và cổng mở
- **Tác dụng:** Tắt máy chủ VNC có sẵn (cổng 5900).
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo launchctl disable system/com.apple.screensharing
sudo launchctl bootout system/com.apple.screensharing
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Cài đặt chung > Chia sẻ > Chia sẻ màn hình: Tắt.
- **Rủi ro nếu không sửa:** Ai tới được cổng 5900 cũng có thể thử xem và điều khiển màn hình.
- **Rủi ro khi sửa:** Bạn không còn điều khiển máy từ xa bằng Chia sẻ màn hình. Với máy không gắn màn hình, hãy giữ bật.
- **Nguồn:** Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến

<a id="airplay"></a>

### `airplay`: Tắt Bộ thu AirPlay

- **Mức:** vừa
- **Nhóm:** Chia sẻ và cổng mở
- **Tác dụng:** Trung tâm điều khiển ngừng mở cổng 5000 và 7000 trên mọi giao diện mạng.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
defaults -currentHost write com.apple.controlcenter AirplayRecieverEnabled -bool false
killall ControlCenter
sleep 3
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Cài đặt chung > AirDrop & Handoff > Bộ thu AirPlay: Tắt.
- **Rủi ro nếu không sửa:** Thiết bị trong mạng dò thấy máy và gửi nội dung tới máy được, đồng thời thêm hai cổng luôn mở.
- **Rủi ro khi sửa:** Bạn không còn AirPlay từ iPhone hay iPad sang máy này được. Trung tâm điều khiển khởi động lại trong giây lát.
- **Nguồn:** Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến

<a id="listeners"></a>

### `listeners`: Không dịch vụ nào khác mở cổng ra mạng

- **Mức:** vừa
- **Nhóm:** Chia sẻ và cổng mở
- **Tác dụng:** Liệt kê mọi tiến trình nhận kết nối TCP trên địa chỉ khác localhost. Không có sudo thì chỉ thấy tiến trình của bạn.
- **Cách sửa (làm tay):** Với từng dịch vụ trong danh sách: cho nó chỉ nghe 127.0.0.1 nếu chỉ máy này cần dùng, không thì tắt. rapportd là Handoff (xem mục handoff).
- **Rủi ro nếu không sửa:** Mỗi dịch vụ mở cổng là một lối vào. Máy chủ phát triển và cơ sở dữ liệu mở trên mọi giao diện thường bị bất kỳ ai cùng mạng truy cập.
- **Rủi ro khi sửa:** Cho dịch vụ chỉ nghe 127.0.0.1 hoặc tắt nó thì thiết bị khác, kể cả điện thoại hay VPN của bạn, không truy cập được nữa.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="file_sharing"></a>

### `file_sharing`: Tắt Chia sẻ tệp

- **Mức:** vừa
- **Nhóm:** Chia sẻ và cổng mở
- **Tác dụng:** Tắt máy chủ tệp SMB (cổng 445) đang chia sẻ thư mục của máy ra mạng.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo launchctl disable system/com.apple.smbd
sudo launchctl bootout system/com.apple.smbd
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Cài đặt chung > Chia sẻ > Chia sẻ tệp: Tắt.
- **Rủi ro nếu không sửa:** Thư mục chia sẻ có thể bị bất kỳ ai trong mạng biết hoặc đoán được mật khẩu tài khoản đọc hay sửa, và SMB là dịch vụ hay bị khai thác.
- **Rủi ro khi sửa:** Máy khác không còn mở được thư mục trên máy này, và các Mac khác không sao lưu Time Machine vào máy này được nữa.
- **Nguồn:** Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến

<a id="internet_sharing"></a>

### `internet_sharing`: Tắt Chia sẻ Internet

- **Mức:** vừa
- **Nhóm:** Chia sẻ và cổng mở
- **Tác dụng:** Đảm bảo máy không đóng vai router chia sẻ kết nối internet cho thiết bị khác.
- **Cách sửa (làm tay):** Cài đặt hệ thống > Cài đặt chung > Chia sẻ > Chia sẻ Internet: Tắt.
- **Rủi ro nếu không sửa:** Thiết bị bạn không kiểm soát có thể đi qua máy này để vào mạng nội bộ, và lưu lượng của chúng chạy qua máy bạn.
- **Rủi ro khi sửa:** Thiết bị đang nhờ máy này để vào internet sẽ mất kết nối.
- **Nguồn:** Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến

<a id="remote_management"></a>

### `remote_management`: Tắt Quản lý từ xa (Apple Remote Desktop)

- **Mức:** vừa
- **Nhóm:** Chia sẻ và cổng mở
- **Tác dụng:** Tắt tác tử Apple Remote Desktop (cổng 3283), thứ cho phép quản trị viên xem, điều khiển và chạy lệnh trên máy.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo /System/Library/CoreServices/RemoteManagement/ARDAgent.app/Contents/Resources/kickstart -deactivate -stop
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Cài đặt chung > Chia sẻ > Quản lý từ xa: Tắt.
- **Rủi ro nếu không sửa:** Ai có được một tài khoản được phép là có thể điều khiển toàn bộ máy qua mạng.
- **Rủi ro khi sửa:** Bộ phận IT hoặc công cụ quản lý máy bằng Apple Remote Desktop không còn kết nối được. Đừng tắt trên máy công ty quản lý mà chưa hỏi.
- **Nguồn:** Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến

<a id="remote_apple_events"></a>

### `remote_apple_events`: Tắt Điều khiển ứng dụng từ xa bằng script

- **Mức:** vừa
- **Nhóm:** Chia sẻ và cổng mở
- **Tác dụng:** Tắt Remote Apple Events (cổng 3031), thứ cho phép script trên máy khác điều khiển app trên máy này.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo systemsetup -setremoteappleevents off || {
  sudo launchctl disable system/com.apple.AEServer
  sudo launchctl bootout system/com.apple.AEServer
}
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Cài đặt chung > Chia sẻ > Điều khiển ứng dụng từ xa: Tắt.
- **Rủi ro nếu không sửa:** Người có mật khẩu một tài khoản có thể điều khiển app của bạn từ xa, ví dụ để đọc dữ liệu hay gửi tin nhắn.
- **Rủi ro khi sửa:** Các script AppleScript chạy từ máy khác để điều khiển máy này sẽ ngừng hoạt động.
- **Nguồn:** Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến

<a id="sudo_password"></a>

### `sudo_password`: sudo luôn đòi mật khẩu

- **Mức:** vừa
- **Nhóm:** Tài khoản và đăng nhập
- **Tác dụng:** Tìm quy tắc NOPASSWD trong /etc/sudoers và /etc/sudoers.d, thứ cho phép chạy lệnh quyền root mà không cần gõ mật khẩu.
- **Cách sửa (làm tay):** Chạy 'sudo visudo' (và 'sudo visudo -f' cho từng file trong /etc/sudoers.d) rồi xoá hoặc thu hẹp các quy tắc NOPASSWD được liệt kê.
- **Rủi ro nếu không sửa:** Bất kỳ chương trình nào chạy dưới tên bạn, kể cả mã độc, đều có thể âm thầm lên quyền root.
- **Rủi ro khi sửa:** Script hoặc công cụ đang dựa vào sudo không mật khẩu sẽ dừng lại và hỏi mật khẩu.
- **Nguồn:** Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến

<a id="ssh_key_passphrase"></a>

### `ssh_key_passphrase`: Khoá SSH riêng được bảo vệ bằng passphrase

- **Mức:** vừa
- **Nhóm:** Tài khoản và đăng nhập
- **Tác dụng:** Kiểm mọi khoá riêng trong ~/.ssh. Khoá nằm trên thiết bị phần cứng (sk-...) được tính là đã bảo vệ.
- **Cách sửa (làm tay):** Với từng khoá trong danh sách, chạy 'ssh-keygen -p -f ~/.ssh/<khoá>' và đặt passphrase.
- **Rủi ro nếu không sửa:** Ai sao chép được file khoá, ví dụ từ bản sao lưu hoặc qua mã độc, là đăng nhập được vào mọi máy chủ tin khoá đó.
- **Rủi ro khi sửa:** Bạn phải gõ passphrase khi dùng khoá. Thêm vào keychain của macOS bằng 'ssh-add --apple-use-keychain' để không phải gõ mỗi lần.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="ssh_host_keys"></a>

### `ssh_host_keys`: SSH có kiểm tra host key của máy chủ

- **Mức:** vừa
- **Nhóm:** Tài khoản và đăng nhập
- **Tác dụng:** Tìm 'StrictHostKeyChecking no' hoặc 'UserKnownHostsFile /dev/null' trong ~/.ssh/config và /etc/ssh/ssh_config, những dòng tắt việc xác minh máy chủ.
- **Cách sửa (làm tay):** Xoá các dòng đó, hoặc chỉ dùng cho một máy thử nghiệm cụ thể thay vì 'Host *'.
- **Rủi ro nếu không sửa:** Người trong mạng có thể giả làm máy chủ và đọc được những gì bạn gửi, kể cả mật khẩu và mã nguồn.
- **Rủi ro khi sửa:** SSH sẽ hỏi bạn xác nhận khoá của máy chủ ở lần kết nối đầu, và cảnh báo nếu khoá đó thay đổi về sau.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="hint"></a>

### `hint`: Không hiện gợi ý mật khẩu

- **Mức:** thấp
- **Nhóm:** Tài khoản và đăng nhập
- **Tác dụng:** Màn hình đăng nhập không hiện gợi ý mật khẩu sau các lần gõ sai.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo defaults write /Library/Preferences/com.apple.loginwindow RetriesUntilHint -int 0
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Màn hình khoá > Hiển thị gợi ý mật khẩu: Tắt.
- **Rủi ro nếu không sửa:** Gợi ý có thể giúp người khác đoán ra mật khẩu.
- **Rủi ro khi sửa:** Nếu quên mật khẩu sẽ không có gợi ý. Hãy lưu nó trong trình quản lý mật khẩu.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="admin_user"></a>

### `admin_user`: Tài khoản dùng hằng ngày là tài khoản thường

- **Mức:** thấp
- **Nhóm:** Tài khoản và đăng nhập
- **Tác dụng:** Tách quyền quản trị ra một tài khoản riêng, để chương trình bạn chạy hằng ngày không thể dùng thông tin của bạn thay đổi toàn hệ thống.
- **Cách sửa (làm tay):** Tạo thêm một tài khoản quản trị trong Cài đặt > Người dùng & Nhóm, đăng nhập vào đó, rồi đổi tài khoản hằng ngày thành Tài khoản thường.
- **Rủi ro nếu không sửa:** Bất kỳ chương trình nào bạn chạy cũng có thể xin sudo hoặc cài phần mềm hệ thống bằng chính mật khẩu bạn gõ hằng ngày.
- **Rủi ro khi sửa:** Bạn phải nhập tên và mật khẩu tài khoản quản trị khi thay đổi hệ thống. Một số app mặc định người dùng là quản trị và có thể không chạy.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="hostname"></a>

### `hostname`: Tên máy không lộ danh tính

- **Mức:** thấp
- **Nhóm:** Tài khoản và đăng nhập
- **Tác dụng:** Tên máy được phát ra mạng nội bộ qua các dịch vụ chia sẻ và dò tìm.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
printf "New computer name (e.g. MacBook) / Tên máy mới: "
read -r new_name
[ -n "$new_name" ] || return 1
host_name=$(printf %s "$new_name" | tr -cs 'A-Za-z0-9-' '-' | sed 's/^-*//; s/-*$//')
sudo scutil --set ComputerName "$new_name"
sudo scutil --set HostName "$host_name"
sudo scutil --set LocalHostName "$host_name"
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Cài đặt chung > Giới thiệu > Tên, hoặc chạy 'sudo scutil --set ComputerName <tên>' (và HostName, LocalHostName).
- **Rủi ro nếu không sửa:** Người dùng chung mạng (quán cà phê, khách sạn, công ty) thấy được tên hoặc tên đăng nhập của bạn.
- **Rủi ro khi sửa:** Thiết bị đang biết máy theo tên cũ (cấu hình SSH, thư mục chia sẻ, danh bạ AirDrop) sẽ thấy tên mới.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="sudo_home"></a>

### `sudo_home`: sudo không giữ biến HOME của bạn

- **Mức:** thấp
- **Nhóm:** Tài khoản và đăng nhập
- **Tác dụng:** macOS giữ nguyên HOME khi dùng sudo, nên chương trình chạy quyền root đọc file cấu hình trong thư mục của bạn. Bản sửa bỏ HOME khỏi env_keep bằng một file riêng trong /etc/sudoers.d, có kiểm cú pháp trước.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
rule_file=$(mktemp)
echo 'Defaults env_keep -= "HOME"' > "$rule_file"
sudo visudo -cf "$rule_file" && sudo install -m 0440 -o root -g wheel "$rule_file" /etc/sudoers.d/macguard-env-home
rm -f "$rule_file"
```

- **Nếu sửa tự động không được:** Chạy 'sudo visudo -f /etc/sudoers.d/macguard-env-home' rồi thêm dòng: Defaults env_keep -= "HOME"
- **Rủi ro nếu không sửa:** Chương trình ghi được vào thư mục của bạn có thể cài sẵn cấu hình để root nạp về sau.
- **Rủi ro khi sửa:** Lệnh chạy bằng sudo dùng thư mục của root (/var/root), nên cấu hình công cụ cá nhân không còn áp dụng khi dùng sudo. Hoàn tác bằng cách xoá /etc/sudoers.d/macguard-env-home.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="signed"></a>

### `signed`: Tường lửa hỏi trước khi cho app đã ký nhận kết nối

- **Mức:** thấp
- **Nhóm:** Mạng và tường lửa
- **Tác dụng:** Mặc định tường lửa tự cho phần mềm có chữ ký của Apple hoặc nhà phát triển đã xác định nhận kết nối. Bản sửa bắt macOS hỏi bạn trước.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setallowsigned off
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setallowsignedapp off
sudo pkill -HUP socketfilterfw
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Mạng > Tường lửa > Tuỳ chọn: tắt tự động cho phép phần mềm có sẵn và phần mềm tải về đã ký.
- **Rủi ro nếu không sửa:** Bất kỳ app có chữ ký nào, kể cả app có chữ ký nhưng độc hại, đều nhận kết nối mà bạn không biết.
- **Rủi ro khi sửa:** Bạn sẽ bị hỏi nhiều hơn. AirDrop ngừng chạy cho tới khi bạn cho phép rapportd và sharingd (xem 'macguard tips').
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="dns"></a>

### `dns`: DNS mã hoá hoặc có lọc

- **Mức:** thấp
- **Nhóm:** Mạng và tường lửa
- **Tác dụng:** Mã hoá truy vấn tên miền bằng DNS profile (Quad9, AdGuard, NextDNS) hoặc bộ phân giải chạy trên máy như dnscrypt-proxy, dnsmasq.
- **Cách sửa (làm tay):** Cài DNS profile của Quad9, AdGuard hoặc NextDNS, hoặc 'brew install dnscrypt-proxy' rồi trỏ DNS về 127.0.0.1.
- **Rủi ro nếu không sửa:** Nhà mạng và người dùng chung mạng có thể xem, ghi lại hoặc sửa mọi tên miền bạn truy cập.
- **Rủi ro khi sửa:** Nhà cung cấp DNS sẽ thấy truy vấn của bạn. Một số VPN và mạng có trang đăng nhập ghi đè hoặc xung đột với DNS tuỳ chỉnh.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="wifi"></a>

### `wifi`: Wi-Fi dùng WPA3

- **Mức:** thấp
- **Nhóm:** Mạng và tường lửa
- **Tác dụng:** WPA3 bảo vệ mật khẩu Wi-Fi khỏi bị đoán ngoại tuyến và mã hoá lưu lượng tốt hơn WPA2.
- **Cách sửa (làm tay):** Đặt router sang WPA3 Personal (hoặc WPA2/WPA3), và bật Địa chỉ Wi-Fi riêng cho từng mạng trong Cài đặt > Wi-Fi > Chi tiết.
- **Rủi ro nếu không sửa:** Người ở gần có thể bắt gói bắt tay và dò mật khẩu Wi-Fi.
- **Rủi ro khi sửa:** Thiết bị cũ chỉ hỗ trợ WPA2 không vào được mạng chỉ dùng WPA3.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="bonjour"></a>

### `bonjour`: Tắt quảng bá Bonjour

- **Mức:** thấp
- **Nhóm:** Mạng và tường lửa
- **Tác dụng:** Máy không còn tự quảng bá bản thân và dịch vụ của nó ra mạng nội bộ qua multicast DNS.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo defaults write /Library/Preferences/com.apple.mDNSResponder NoMulticastAdvertisements -bool true
```

- **Nếu sửa tự động không được:** Chạy 'sudo defaults write /Library/Preferences/com.apple.mDNSResponder NoMulticastAdvertisements -bool YES' rồi khởi động lại.
- **Rủi ro nếu không sửa:** Mọi thiết bị trong mạng thấy tên máy và các dịch vụ máy đang mở.
- **Rủi ro khi sửa:** AirPlay, AirPrint, việc dò thấy máy qua AirDrop và chia sẻ file tới máy này ngừng chạy. Có hiệu lực sau khi khởi động lại. Hoàn tác: sudo defaults delete /Library/Preferences/com.apple.mDNSResponder NoMulticastAdvertisements
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="handoff"></a>

### `handoff`: Tắt Handoff

- **Mức:** thấp
- **Nhóm:** Mạng và tường lửa
- **Tác dụng:** Máy không còn phát hoạt động hiện tại của bạn tới thiết bị Apple ở gần qua Bluetooth và mạng.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
defaults -currentHost write com.apple.coreservices.useractivityd ActivityAdvertisingAllowed -bool false
defaults -currentHost write com.apple.coreservices.useractivityd ActivityReceivingAllowed -bool false
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Cài đặt chung > AirDrop & Handoff > Cho phép Handoff giữa máy Mac này và các thiết bị iCloud: Tắt.
- **Rủi ro nếu không sửa:** Thiết bị ở gần dò thấy máy, và rapportd giữ một cổng mạng luôn mở.
- **Rủi ro khi sửa:** Mất tính năng làm tiếp việc giữa các thiết bị và Bộ nhớ tạm chung. Đăng xuất rồi đăng nhập lại để có hiệu lực đầy đủ.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="brew_analytics"></a>

### `brew_analytics`: Tắt thống kê của Homebrew

- **Mức:** thấp
- **Nhóm:** Phần mềm
- **Tác dụng:** Homebrew ngừng gửi dữ liệu sử dụng ẩn danh về máy chủ thống kê.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
brew analytics off
```

- **Nếu sửa tự động không được:** Chạy 'brew analytics off'.
- **Rủi ro nếu không sửa:** Mỗi lần cài và chạy lệnh đều gửi sự kiện sử dụng của máy bạn đi.
- **Rủi ro khi sửa:** Không có rủi ro với bạn. Nhóm phát triển Homebrew có ít dữ liệu hơn để ưu tiên công việc.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="brew_redirect"></a>

### `brew_redirect`: Homebrew từ chối chuyển hướng HTTPS sang HTTP

- **Mức:** thấp
- **Nhóm:** Phần mềm
- **Tác dụng:** Đặt HOMEBREW_NO_INSECURE_REDIRECT=1 trong ~/.zprofile để bản tải về không bị hạ từ HTTPS xuống HTTP.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sh -c 'grep -qx "export HOMEBREW_NO_INSECURE_REDIRECT=1" ~/.zprofile 2>/dev/null || echo "export HOMEBREW_NO_INSECURE_REDIRECT=1" >> ~/.zprofile'
```

- **Nếu sửa tự động không được:** Thêm 'export HOMEBREW_NO_INSECURE_REDIRECT=1' vào file khởi động của shell.
- **Rủi ro nếu không sửa:** Một lần chuyển hướng sang HTTP thường cho phép người trong mạng sửa file đang tải.
- **Rủi ro khi sửa:** Vài gói có nguồn chuyển hướng sang HTTP sẽ tải lỗi. Áp dụng cho phiên zsh mới. Hoàn tác bằng cách xoá dòng đó trong ~/.zprofile.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="diagnostics"></a>

### `diagnostics`: Không chia sẻ dữ liệu chẩn đoán và sử dụng

- **Mức:** thấp
- **Nhóm:** Quyền riêng tư
- **Tác dụng:** Ngừng gửi dữ liệu phân tích và báo lỗi về Apple và nhà phát triển app.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo defaults write "/Library/Application Support/CrashReporter/DiagnosticMessagesHistory.plist" AutoSubmit -bool false
sudo defaults write "/Library/Application Support/CrashReporter/DiagnosticMessagesHistory.plist" ThirdPartyDataSubmit -bool false
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Quyền riêng tư & Bảo mật > Phân tích & Cải tiến: tắt hết.
- **Rủi ro nếu không sửa:** Thông tin sử dụng, lỗi và cấu hình máy được gửi đi định kỳ.
- **Rủi ro khi sửa:** Apple và nhà phát triển có ít dữ liệu hơn để sửa lỗi bạn gặp.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="crash_dialog"></a>

### `crash_dialog`: Tắt hộp thoại gửi báo cáo lỗi

- **Mức:** thấp
- **Nhóm:** Quyền riêng tư
- **Tác dụng:** Tắt hộp thoại đề nghị gửi báo cáo lỗi cho Apple sau khi app bị treo.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
defaults write com.apple.CrashReporter DialogType none
```

- **Nếu sửa tự động không được:** Chạy 'defaults write com.apple.CrashReporter DialogType none'.
- **Rủi ro nếu không sửa:** Một cú bấm nhầm là gửi đi báo cáo có thể chứa thông tin công việc của bạn.
- **Rủi ro khi sửa:** Bạn không còn được báo khi app chạy nền bị treo. Báo cáo vẫn được lưu trên máy trong Console.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="icloud_save"></a>

### `icloud_save`: Tài liệu mới mặc định lưu trên máy

- **Mức:** thấp
- **Nhóm:** Quyền riêng tư
- **Tác dụng:** Hộp thoại lưu mặc định chọn máy Mac thay vì iCloud Drive.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
defaults write NSGlobalDomain NSDocumentSaveNewDocumentsToCloud -bool false
```

- **Nếu sửa tự động không được:** Chạy 'defaults write NSGlobalDomain NSDocumentSaveNewDocumentsToCloud -bool false'.
- **Rủi ro nếu không sửa:** Tài liệu có thể lên iCloud mà bạn không để ý, nằm ngoài tầm kiểm soát và, nếu không bật Bảo vệ dữ liệu nâng cao, Apple đọc được.
- **Rủi ro khi sửa:** Tài liệu mới không tự đồng bộ sang thiết bị khác trừ khi bạn tự chọn iCloud Drive.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="extensions"></a>

### `extensions`: Finder hiện mọi phần mở rộng của tên file

- **Mức:** thấp
- **Nhóm:** Quyền riêng tư
- **Tác dụng:** Hiện đúng loại file, ví dụ 'hoadon.pdf.app' thay vì 'hoadon.pdf'.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
defaults write NSGlobalDomain AppleShowAllExtensions -bool true
killall Finder
```

- **Nếu sửa tự động không được:** Finder > Cài đặt > Nâng cao > Hiển thị tất cả phần mở rộng tên tệp.
- **Rủi ro nếu không sửa:** App hoặc script độc hại có thể giả dạng tài liệu.
- **Rủi ro khi sửa:** Tên file trông dài hơn. Finder khởi động lại và đóng các cửa sổ đang mở.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="hidden_files"></a>

### `hidden_files`: Finder hiện file ẩn và thư mục ~/Library

- **Mức:** thấp
- **Nhóm:** Quyền riêng tư
- **Tác dụng:** Hiện file ẩn và thư mục Library, nơi chứa tiến trình nền và dữ liệu app.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
defaults write com.apple.finder AppleShowAllFiles -bool true
chflags nohidden "$TARGET_HOME/Library"
killall Finder
```

- **Nếu sửa tự động không được:** Trong Finder bấm Command-Shift-Dấu chấm, hoặc chạy 'defaults write com.apple.finder AppleShowAllFiles -bool true'.
- **Rủi ro nếu không sửa:** File do mã độc hoặc app thả vào chỗ ẩn sẽ không bị nhìn thấy.
- **Rủi ro khi sửa:** Thư mục trông rối hơn và dễ xoá nhầm file của hệ thống hoặc app. Finder sẽ khởi động lại.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="umask"></a>

### `umask`: File mới mặc định chỉ mình bạn đọc được (umask 077)

- **Mức:** thấp
- **Nhóm:** Quyền riêng tư
- **Tác dụng:** File và thư mục do app tạo ra chỉ bạn đọc được, thay vì mọi người dùng trên máy.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo launchctl config user umask 077
```

- **Nếu sửa tự động không được:** Chạy 'sudo launchctl config user umask 077' rồi khởi động lại.
- **Rủi ro nếu không sửa:** Tài khoản và dịch vụ khác trên máy đọc được phần lớn file bạn tạo.
- **Rủi ro khi sửa:** Có hiệu lực sau khi khởi động lại. File muốn chia sẻ với người dùng hoặc nhóm khác trên máy phải tự đổi quyền, và vài trình cài đặt cần mặc định cũ. Hoàn tác: sudo launchctl config user umask 022
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="terminal_keyboard"></a>

### `terminal_keyboard`: Bật Nhập bàn phím an toàn trong Terminal

- **Mức:** thấp
- **Nhóm:** Quyền riêng tư
- **Tác dụng:** Ngăn app khác đọc những gì bạn gõ trong Terminal.app, kể cả mật khẩu.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
defaults write com.apple.Terminal SecureKeyboardEntry -bool true
```

- **Nếu sửa tự động không được:** Terminal > Nhập bàn phím an toàn.
- **Rủi ro nếu không sửa:** App ghi phím có quyền theo dõi đầu vào có thể bắt được mật khẩu gõ trong Terminal.
- **Rủi ro khi sửa:** Công cụ gõ tắt và một số công cụ tự động hoá ngừng chạy trong Terminal. Có hiệu lực khi mở lại Terminal. Terminal khác (iTerm2, Ghostty) có cài đặt riêng.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="printer_sharing"></a>

### `printer_sharing`: Tắt Chia sẻ máy in và quản trị máy in từ xa

- **Mức:** thấp
- **Nhóm:** Chia sẻ và cổng mở
- **Tác dụng:** CUPS, hệ thống in của máy, ngừng chia sẻ máy in và ngừng nhận lệnh quản trị hay lệnh in từ máy khác.
- **Lệnh `macguard fix` sẽ chạy:**

```bash
sudo cupsctl --no-share-printers --no-remote-admin --no-remote-any
```

- **Nếu sửa tự động không được:** Cài đặt hệ thống > Cài đặt chung > Chia sẻ > Chia sẻ máy in: Tắt.
- **Rủi ro nếu không sửa:** Máy khác gửi lệnh in tới máy in của bạn hoặc truy cập giao diện web của CUPS được, nơi từng có lỗ hổng khai thác từ xa.
- **Rủi ro khi sửa:** Máy in cắm vào máy này không còn dùng chung được với máy khác.
- **Nguồn:** Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến

<a id="media_sharing"></a>

### `media_sharing`: Tắt Chia sẻ phương tiện

- **Mức:** thấp
- **Nhóm:** Chia sẻ và cổng mở
- **Tác dụng:** Ngừng chia sẻ thư viện nhạc và video với thiết bị khác trong mạng (cổng 3689).
- **Cách sửa (làm tay):** Cài đặt hệ thống > Cài đặt chung > Chia sẻ > Chia sẻ phương tiện: Tắt.
- **Rủi ro nếu không sửa:** Thiết bị cùng mạng có thể xem thư viện nhạc, video và suy ra thông tin về bạn.
- **Rủi ro khi sửa:** Apple TV và máy khác không còn phát được nội dung từ máy này.
- **Nguồn:** Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến

<a id="ssh_key_strength"></a>

### `ssh_key_strength`: Khoá SSH dùng thuật toán đủ mạnh

- **Mức:** thấp
- **Nhóm:** Tài khoản và đăng nhập
- **Tác dụng:** Đánh dấu khoá DSA và khoá RSA ngắn hơn 3072 bit. Khoá Ed25519, ECDSA và khoá trên thiết bị phần cứng được tính là đạt.
- **Cách sửa (làm tay):** Tạo khoá mới bằng 'ssh-keygen -t ed25519', thêm vào các máy chủ và dịch vụ, rồi bỏ khoá cũ.
- **Rủi ro nếu không sửa:** Khoá yếu dễ bị phá hơn và ngày càng nhiều máy chủ từ chối.
- **Rủi ro khi sửa:** Phải thêm khoá mới vào mọi máy chủ và dịch vụ đang tin khoá cũ trước khi bỏ khoá cũ.
- **Nguồn:** Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến

<a id="ssh_agent_forwarding"></a>

### `ssh_agent_forwarding`: Tắt chuyển tiếp SSH agent

- **Mức:** thấp
- **Nhóm:** Tài khoản và đăng nhập
- **Tác dụng:** Tìm 'ForwardAgent yes' trong ~/.ssh/config và /etc/ssh/ssh_config.
- **Cách sửa (làm tay):** Xoá 'ForwardAgent yes', hoặc chỉ bật cho một máy chủ cụ thể bạn tin cậy.
- **Rủi ro nếu không sửa:** Quản trị viên hoặc kẻ tấn công trên máy chủ bạn kết nối có thể dùng khoá của bạn để đăng nhập nơi khác dưới tên bạn khi phiên còn mở.
- **Rủi ro khi sửa:** Việc nhảy từ máy chủ này sang máy chủ khác bằng khoá trên máy bạn sẽ không chạy nữa. Hãy dùng 'ssh -J' (ProxyJump), cách này không lộ agent.
- **Nguồn:** Không có trong tài liệu; macguard thêm vào như một bước gia cố phổ biến

<a id="lockdown"></a>

### `lockdown`: Chế độ phong toả (Lockdown Mode)

- **Mức:** tham khảo
- **Nhóm:** Hệ thống
- **Tác dụng:** Tắt các tính năng hay bị tấn công có chủ đích lợi dụng (một số công nghệ web, tệp đính kèm, lời mời đến, phụ kiện cắm dây).
- **Cách sửa (làm tay):** Cài đặt hệ thống > Quyền riêng tư & Bảo mật > Chế độ phong toả > Bật.
- **Rủi ro nếu không sửa:** Chỉ đáng lo nếu bạn có thể bị nhắm tới riêng, ví dụ nhà báo, nhà hoạt động hay lãnh đạo doanh nghiệp.
- **Rủi ro khi sửa:** Nhiều trang web và tính năng chạy kém đi. Có thể loại trừ trang tin cậy trong Safari.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="outbound_fw"></a>

### `outbound_fw`: Có app tường lửa kiểm soát kết nối đi ra

- **Mức:** tham khảo
- **Nhóm:** Mạng và tường lửa
- **Tác dụng:** App như LuLu, Little Snitch hay Radio Silence cho thấy và chặn kết nối đi ra, việc tường lửa có sẵn không làm được.
- **Cách sửa (làm tay):** Cài LuLu (miễn phí, mã nguồn mở) hoặc Little Snitch rồi xem lại luật của nó. pf (xem 'macguard tips') là lựa chọn nâng cao.
- **Rủi ro nếu không sửa:** Bạn không thấy và không chặn được app hay mã độc gửi dữ liệu ra ngoài.
- **Rủi ro khi sửa:** Lúc đầu bị hỏi cho phép hay chặn rất nhiều. Các app này cài một system extension có quyền rộng trên mạng.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="hosts_blocklist"></a>

### `hosts_blocklist`: File hosts chặn tên miền xấu đã biết

- **Mức:** tham khảo
- **Nhóm:** Mạng và tường lửa
- **Tác dụng:** Danh sách chặn trong /etc/hosts ngăn mọi app kết nối tới tên miền mã độc, quảng cáo và theo dõi đã biết.
- **Cách sửa (làm tay):** Thêm một danh sách như StevenBlack/hosts: curl https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts | sudo tee -a /etc/hosts
- **Rủi ro nếu không sửa:** App vẫn kết nối được tới các tên miền đó nếu không có lớp khác (DNS lọc, tường lửa chiều ra) chặn.
- **Rủi ro khi sửa:** Danh sách quá rộng có thể làm hỏng vài trang web. Luôn giữ dòng '127.0.0.1 localhost'.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="bluetooth"></a>

### `bluetooth`: Tắt Bluetooth khi không dùng

- **Mức:** tham khảo
- **Nhóm:** Mạng và tường lửa
- **Tác dụng:** Tắt Bluetooth bỏ đi một bề mặt tấn công không dây và một cách theo dõi thiết bị.
- **Cách sửa (làm tay):** Trung tâm điều khiển hoặc Cài đặt hệ thống > Bluetooth: Tắt.
- **Rủi ro nếu không sửa:** Lỗ hổng Bluetooth và việc bị theo dõi vẫn có thể xảy ra trong tầm sóng.
- **Rủi ro khi sửa:** Bàn phím, chuột, tai nghe Bluetooth, AirDrop và Handoff ngừng chạy. Đừng tắt nếu bàn phím duy nhất của bạn là Bluetooth.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="launch_items"></a>

### `launch_items`: Đã rà soát tiến trình nền của bên thứ ba

- **Mức:** tham khảo
- **Nhóm:** Phần mềm
- **Tác dụng:** Liệt kê launch agent và daemon không phải của Apple. Đây là cách mã độc hay dùng nhất để tồn tại sau khi khởi động lại.
- **Cách sửa (làm tay):** Xem từng mục trong Cài đặt > Cài đặt chung > Mục đăng nhập & Tiện ích mở rộng và tắt mục bạn không nhận ra. BlockBlock có thể cảnh báo khi có mục mới.
- **Rủi ro nếu không sửa:** Phần mềm không mong muốn hoặc độc hại tiếp tục chạy ngầm mà bạn không hay biết.
- **Rủi ro khi sửa:** Gỡ một mục bạn cần sẽ làm dừng trình cập nhật, đồng bộ hoặc tiến trình phụ của app đó.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="sysext"></a>

### `sysext`: Đã rà soát system extension

- **Mức:** tham khảo
- **Nhóm:** Phần mềm
- **Tác dụng:** Liệt kê system extension đang chạy (lọc mạng, bảo mật đầu cuối, driver). Chúng chạy với quyền rất rộng.
- **Cách sửa (làm tay):** Xem trong Cài đặt > Cài đặt chung > Mục đăng nhập & Tiện ích mở rộng, và gỡ app bạn không cần.
- **Rủi ro nếu không sửa:** Extension bạn không còn dùng hoặc không tin vẫn có thể xem hoặc sửa lưu lượng mạng và sự kiện hệ thống.
- **Rủi ro khi sửa:** Gỡ extension sẽ làm app đã cài nó (VPN, tường lửa, diệt virus) ngừng hoạt động.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh

<a id="app_runtime"></a>

### `app_runtime`: App dùng Hardened Runtime

- **Mức:** tham khảo
- **Nhóm:** Phần mềm
- **Tác dụng:** Liệt kê app trong /Applications không bật Hardened Runtime, cơ chế chặn chèn mã và can thiệp lúc chạy.
- **Cách sửa (làm tay):** Ưu tiên bản App Store hoặc bản đã được chứng nhận của các app trong danh sách, cập nhật chúng, hoặc thay bằng app khác.
- **Rủi ro nếu không sửa:** Mã độc dễ chiếm quyền các app này hơn, ví dụ bằng cách chèn thư viện.
- **Rủi ro khi sửa:** Thay app bằng app khác có thể mất tính năng. Không có gì bị tự động thay đổi.
- **Nguồn:** Tài liệu macOS Security and Privacy Guide của drduh
