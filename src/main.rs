use slint::SharedString;
use std::path::Path;
use std::sync::{Arc, Mutex};
use windows::Win32::Media::Audio::{PlaySoundW, SND_ASYNC, SND_FILENAME, SND_LOOP, SND_SYNC};
use windows::Win32::Media::Multimedia::{mciGetErrorStringW, mciSendStringW};
use windows::core::{PCWSTR, PWSTR};

slint::slint! {
    export component MainWindow inherits Window {
        title: "多媒体播放器 🎵🎬";
        preferred-width: 600px;
        preferred-height: 700px;

        // PlaySound API 回调
        callback play-sync();
        callback play-async();
        callback play-loop();
        callback stop-sound();

        // MCI API 回调
        callback mci-open();
        callback mci-play();
        callback mci-pause();
        callback mci-resume();
        callback mci-stop();
        callback mci-close();
        callback mci-record();
        callback mci-save-recording();

        in-out property <string> status: "准备就绪";
        in-out property <string> sound-path: "C:\\Windows\\Media\\chimes.wav";
        in-out property <string> record-path: "C:\\recording.wav";
        in-out property <string> mci-status: "MCI 未打开";
        in-out property <bool> is-recording: false;

        VerticalLayout {
            padding: 20px;
            spacing: 15px;

            // 标题
            Text {
                text: "🎵 Windows 多媒体播放器";
                font-size: 28px;
                font-weight: 700;
                horizontal-alignment: center;
                color: #2c3e50;
            }

            // ============ PlaySound API 部分 ============
            Rectangle {
                background: #ecf0f1;
                border-radius: 12px;

                VerticalLayout {
                    padding: 15px;
                    spacing: 10px;

                    Text {
                        text: "📻 PlaySound API (仅支持 WAV)";
                        font-size: 18px;
                        font-weight: 600;
                        color: #596b7cff;
                    }

                    // 文件路径输入
                    HorizontalLayout {
                        spacing: 10px;
                        Text {
                            text: "文件:";
                            vertical-alignment: center;
                            min-width: 50px;
                        }
                        TextInput {
                            text <=> sound-path;
                            font-size: 12px;
                        }
                    }

                    // PlaySound 按钮组
                    GridLayout {
                        spacing: 8px;
                        Row {
                            Rectangle {
                                background: #27ae60;
                                border-radius: 8px;
                                TouchArea {
                                    clicked => { play-sync(); }
                                }
                                Text {
                                    text: "🔊 同步播放";
                                    color: white;
                                    font-size: 13px;
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                height: 50px;
                            }

                            Rectangle {
                                background: #3498db;
                                border-radius: 8px;
                                TouchArea {
                                    clicked => { play-async(); }
                                }
                                Text {
                                    text: "▶️ 异步播放";
                                    color: white;
                                    font-size: 13px;
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                height: 50px;
                            }
                        }

                        Row {
                            Rectangle {
                                background: #e67e22;
                                border-radius: 8px;
                                TouchArea {
                                    clicked => { play-loop(); }
                                }
                                Text {
                                    text: "🔁 循环播放";
                                    color: white;
                                    font-size: 13px;
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                height: 50px;
                            }

                            Rectangle {
                                background: #c0392b;
                                border-radius: 8px;
                                TouchArea {
                                    clicked => { stop-sound(); }
                                }
                                Text {
                                    text: "⏹️ 停止";
                                    color: white;
                                    font-size: 13px;
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                height: 50px;
                            }
                        }
                    }

                    // PlaySound 状态
                    Rectangle {
                        background: white;
                        border-radius: 6px;
                        border-width: 1px;
                        border-color: #bdc3c7;
                        height: 50px;

                        Text {
                            text: status;
                            font-size: 13px;
                            color: #2c3e50;
                            horizontal-alignment: center;
                            vertical-alignment: center;
                        }
                    }
                }
            }

            // ============ MCI API 部分 ============
            Rectangle {
                background: #e8f5e9;
                border-radius: 12px;

                VerticalLayout {
                    padding: 15px;
                    spacing: 10px;

                    Text {
                        text: "🎬 MCI API (WAV/MIDI/MP3)";
                        font-size: 18px;
                        font-weight: 600;
                        color: #1b5e20;
                    }

                    // MCI 文件路径
                    HorizontalLayout {
                        spacing: 10px;
                        Text {
                            text: "文件:";
                            vertical-alignment: center;
                            min-width: 50px;
                        }
                        TextInput {
                            text <=> sound-path;
                            font-size: 12px;
                        }
                    }

                    // MCI 控制按钮
                    GridLayout {
                        spacing: 8px;
                        Row {
                            Rectangle {
                                background: #4caf50;
                                border-radius: 8px;
                                TouchArea {
                                    clicked => { mci-open(); }
                                }
                                Text {
                                    text: "📂 打开";
                                    color: white;
                                    font-size: 13px;
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                height: 50px;
                            }

                            Rectangle {
                                background: #2196f3;
                                border-radius: 8px;
                                TouchArea {
                                    clicked => { mci-play(); }
                                }
                                Text {
                                    text: "▶️ 播放";
                                    color: white;
                                    font-size: 13px;
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                height: 50px;
                            }

                            Rectangle {
                                background: #ff9800;
                                border-radius: 8px;
                                TouchArea {
                                    clicked => { mci-pause(); }
                                }
                                Text {
                                    text: "⏸️ 暂停";
                                    color: white;
                                    font-size: 13px;
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                height: 50px;
                            }
                        }

                        Row {
                            Rectangle {
                                background: #9c27b0;
                                border-radius: 8px;
                                TouchArea {
                                    clicked => { mci-resume(); }
                                }
                                Text {
                                    text: "⏯️ 继续";
                                    color: white;
                                    font-size: 13px;
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                height: 50px;
                            }

                            Rectangle {
                                background: #f44336;
                                border-radius: 8px;
                                TouchArea {
                                    clicked => { mci-stop(); }
                                }
                                Text {
                                    text: "⏹️ 停止";
                                    color: white;
                                    font-size: 13px;
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                height: 50px;
                            }

                            Rectangle {
                                background: #607d8b;
                                border-radius: 8px;
                                TouchArea {
                                    clicked => { mci-close(); }
                                }
                                Text {
                                    text: "❌ 关闭";
                                    color: white;
                                    font-size: 13px;
                                    horizontal-alignment: center;
                                    vertical-alignment: center;
                                }
                                height: 50px;
                            }
                        }
                    }

                    // MCI 状态
                    Rectangle {
                        background: white;
                        border-radius: 6px;
                        border-width: 1px;
                        border-color: #a5d6a7;
                        height: 50px;

                        Text {
                            text: mci-status;
                            font-size: 13px;
                            color: #1b5e20;
                            horizontal-alignment: center;
                            vertical-alignment: center;
                        }
                    }
                }
            }

            // ============ MCI 录音部分 ============
            Rectangle {
                background: #fff3e0;
                border-radius: 12px;

                VerticalLayout {
                    padding: 15px;
                    spacing: 10px;

                    Text {
                        text: "🎙️ MCI 录音 (仅 WAV)";
                        font-size: 18px;
                        font-weight: 600;
                        color: #e65100;
                    }

                    // 录音文件路径
                    HorizontalLayout {
                        spacing: 10px;
                        Text {
                            text: "保存:";
                            vertical-alignment: center;
                            min-width: 50px;
                        }
                        TextInput {
                            text <=> record-path;
                            font-size: 12px;
                        }
                    }

                    // 录音按钮
                    HorizontalLayout {
                        spacing: 8px;
                        Rectangle {
                            background: is-recording ? #f44336 : #ff5722;
                            border-radius: 8px;
                            TouchArea {
                                clicked => { mci-record(); }
                            }
                            Text {
                                text: is-recording ? "⏺️ 录音中..." : "⏺️ 开始录音";
                                color: white;
                                font-size: 14px;
                                horizontal-alignment: center;
                                vertical-alignment: center;
                            }
                            height: 50px;
                        }

                        Rectangle {
                            background: #4caf50;
                            border-radius: 8px;
                            TouchArea {
                                clicked => { mci-save-recording(); }
                            }
                            Text {
                                text: "💾 停止并保存";
                                color: white;
                                font-size: 14px;
                                horizontal-alignment: center;
                                vertical-alignment: center;
                            }
                            height: 50px;
                        }
                    }
                }
            }

            // 说明文字
            Text {
                text: "💡 支持格式: WAV (PlaySound/MCI), MIDI (MCI), MP3 (MCI)\n📝 MCI API 提供完整的播放控制和录音功能";
                font-size: 11px;
                color: #7f8c8d;
                horizontal-alignment: center;
            }
        }
    }
}

// ============ PlaySound API 函数 ============
fn play_sound_sync(path: &str) -> Result<(), String> {
    if !Path::new(path).exists() {
        return Err(format!("文件不存在: {}", path));
    }

    let path_wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe { PlaySoundW(PCWSTR(path_wide.as_ptr()), None, SND_FILENAME | SND_SYNC) };

    Ok(())
}

fn play_sound_async(path: &str) -> Result<(), String> {
    if !Path::new(path).exists() {
        return Err(format!("文件不存在: {}", path));
    }

    let path_wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe { PlaySoundW(PCWSTR(path_wide.as_ptr()), None, SND_FILENAME | SND_ASYNC) };

    Ok(())
}

fn play_sound_loop(path: &str) -> Result<(), String> {
    if !Path::new(path).exists() {
        return Err(format!("文件不存在: {}", path));
    }

    let path_wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        PlaySoundW(
            PCWSTR(path_wide.as_ptr()),
            None,
            SND_FILENAME | SND_ASYNC | SND_LOOP,
        )
    };

    Ok(())
}

fn stop_sound() {
    unsafe {
        PlaySoundW(PCWSTR::null(), None, Default::default());
    }
}

// ============ MCI API 函数 ============
struct MCIDevice {
    alias: String,
    is_open: bool,
    is_recording: bool,
}

impl MCIDevice {
    fn new() -> Self {
        MCIDevice {
            alias: "media_device".to_string(),
            is_open: false,
            is_recording: false,
        }
    }
}

fn mci_send_string(command: &str) -> Result<String, String> {
    let command_wide: Vec<u16> = command.encode_utf16().chain(std::iter::once(0)).collect();
    let mut return_buffer = vec![0u16; 512];

    unsafe {
        let error = mciSendStringW(
            PCWSTR(command_wide.as_ptr()),
            Some(&mut return_buffer),
            None,
        );

        if error != 0 {
            let mut error_buffer = vec![0u16; 256];
            mciGetErrorStringW(error, &mut error_buffer);

            let error_msg = String::from_utf16_lossy(&error_buffer)
                .trim_end_matches('\0')
                .to_string();
            return Err(format!("MCI 错误 {}: {}", error, error_msg));
        }

        let result = String::from_utf16_lossy(&return_buffer)
            .trim_end_matches('\0')
            .to_string();
        Ok(result)
    }
}

fn get_device_type(path: &str) -> &str {
    let lower_path = path.to_lowercase();
    if lower_path.ends_with(".wav") {
        "waveaudio"
    } else if lower_path.ends_with(".mid") || lower_path.ends_with(".midi") {
        "sequencer"
    } else if lower_path.ends_with(".mp3") {
        "mpegvideo"
    } else {
        "waveaudio" // 默认
    }
}

fn mci_open_file(path: &str, alias: &str) -> Result<String, String> {
    if !Path::new(path).exists() {
        return Err(format!("文件不存在: {}", path));
    }

    let device_type = get_device_type(path);
    let command = format!("open \"{}\" type {} alias {}", path, device_type, alias);
    mci_send_string(&command)?;
    Ok(format!("已打开: {} ({})", path, device_type))
}

fn mci_play(alias: &str) -> Result<String, String> {
    let command = format!("play {}", alias);
    mci_send_string(&command)?;
    Ok("播放中...".to_string())
}

fn mci_pause(alias: &str) -> Result<String, String> {
    let command = format!("pause {}", alias);
    mci_send_string(&command)?;
    Ok("已暂停".to_string())
}

fn mci_resume(alias: &str) -> Result<String, String> {
    let command = format!("resume {}", alias);
    mci_send_string(&command)?;
    Ok("继续播放...".to_string())
}

fn mci_stop(alias: &str) -> Result<String, String> {
    let command = format!("stop {}", alias);
    mci_send_string(&command)?;
    Ok("已停止".to_string())
}

fn mci_close(alias: &str) -> Result<String, String> {
    let command = format!("close {}", alias);
    mci_send_string(&command)?;
    Ok("已关闭设备".to_string())
}

fn mci_record(alias: &str) -> Result<String, String> {
    // 先打开录音设备
    let command = format!("open new type waveaudio alias {}", alias);
    mci_send_string(&command)?;

    // 开始录音
    let command = format!("record {}", alias);
    mci_send_string(&command)?;
    Ok("录音中...".to_string())
}

fn mci_save_recording(alias: &str, save_path: &str) -> Result<String, String> {
    // 停止录音
    let command = format!("stop {}", alias);
    mci_send_string(&command)?;

    // 保存文件
    let command = format!("save {} \"{}\"", alias, save_path);
    mci_send_string(&command)?;

    // 关闭设备
    let command = format!("close {}", alias);
    mci_send_string(&command)?;

    Ok(format!("录音已保存: {}", save_path))
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;

    let mci_device = Arc::new(Mutex::new(MCIDevice::new()));

    // ============ PlaySound 回调 ============
    let ui_handle = ui.as_weak();
    ui.on_play_sync(move || {
        let ui = ui_handle.unwrap();
        let path = ui.get_sound_path().to_string();
        ui.set_status(SharedString::from("🔊 同步播放中..."));

        match play_sound_sync(&path) {
            Ok(_) => ui.set_status(SharedString::from("✅ 播放完成")),
            Err(e) => ui.set_status(SharedString::from(format!("❌ {}", e))),
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_play_async(move || {
        let ui = ui_handle.unwrap();
        let path = ui.get_sound_path().to_string();

        match play_sound_async(&path) {
            Ok(_) => ui.set_status(SharedString::from("▶️ 异步播放中...")),
            Err(e) => ui.set_status(SharedString::from(format!("❌ {}", e))),
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_play_loop(move || {
        let ui = ui_handle.unwrap();
        let path = ui.get_sound_path().to_string();

        match play_sound_loop(&path) {
            Ok(_) => ui.set_status(SharedString::from("🔁 循环播放中...")),
            Err(e) => ui.set_status(SharedString::from(format!("❌ {}", e))),
        }
    });

    let ui_handle = ui.as_weak();
    ui.on_stop_sound(move || {
        let ui = ui_handle.unwrap();
        stop_sound();
        ui.set_status(SharedString::from("⏹️ 已停止播放"));
    });

    // ============ MCI 回调 ============
    let ui_handle = ui.as_weak();
    let device = Arc::clone(&mci_device);
    ui.on_mci_open(move || {
        let ui = ui_handle.unwrap();
        let path = ui.get_sound_path().to_string();
        let mut dev = device.lock().unwrap();

        match mci_open_file(&path, &dev.alias) {
            Ok(msg) => {
                dev.is_open = true;
                ui.set_mci_status(SharedString::from(msg));
            }
            Err(e) => ui.set_mci_status(SharedString::from(format!("❌ {}", e))),
        }
    });

    let ui_handle = ui.as_weak();
    let device = Arc::clone(&mci_device);
    ui.on_mci_play(move || {
        let ui = ui_handle.unwrap();
        let dev = device.lock().unwrap();

        if !dev.is_open {
            ui.set_mci_status(SharedString::from("❌ 请先打开文件"));
            return;
        }

        match mci_play(&dev.alias) {
            Ok(msg) => ui.set_mci_status(SharedString::from(msg)),
            Err(e) => ui.set_mci_status(SharedString::from(format!("❌ {}", e))),
        }
    });

    let ui_handle = ui.as_weak();
    let device = Arc::clone(&mci_device);
    ui.on_mci_pause(move || {
        let ui = ui_handle.unwrap();
        let dev = device.lock().unwrap();

        match mci_pause(&dev.alias) {
            Ok(msg) => ui.set_mci_status(SharedString::from(msg)),
            Err(e) => ui.set_mci_status(SharedString::from(format!("❌ {}", e))),
        }
    });

    let ui_handle = ui.as_weak();
    let device = Arc::clone(&mci_device);
    ui.on_mci_resume(move || {
        let ui = ui_handle.unwrap();
        let dev = device.lock().unwrap();

        match mci_resume(&dev.alias) {
            Ok(msg) => ui.set_mci_status(SharedString::from(msg)),
            Err(e) => ui.set_mci_status(SharedString::from(format!("❌ {}", e))),
        }
    });

    let ui_handle = ui.as_weak();
    let device = Arc::clone(&mci_device);
    ui.on_mci_stop(move || {
        let ui = ui_handle.unwrap();
        let dev = device.lock().unwrap();

        match mci_stop(&dev.alias) {
            Ok(msg) => ui.set_mci_status(SharedString::from(msg)),
            Err(e) => ui.set_mci_status(SharedString::from(format!("❌ {}", e))),
        }
    });

    let ui_handle = ui.as_weak();
    let device = Arc::clone(&mci_device);
    ui.on_mci_close(move || {
        let ui = ui_handle.unwrap();
        let mut dev = device.lock().unwrap();

        match mci_close(&dev.alias) {
            Ok(msg) => {
                dev.is_open = false;
                ui.set_mci_status(SharedString::from(msg));
            }
            Err(e) => ui.set_mci_status(SharedString::from(format!("❌ {}", e))),
        }
    });

    // ============ MCI 录音回调 ============
    let ui_handle = ui.as_weak();
    let device = Arc::clone(&mci_device);
    ui.on_mci_record(move || {
        let ui = ui_handle.unwrap();
        let mut dev = device.lock().unwrap();

        if dev.is_recording {
            ui.set_mci_status(SharedString::from("⚠️ 已经在录音中"));
            return;
        }

        match mci_record("recorder") {
            Ok(msg) => {
                dev.is_recording = true;
                ui.set_is_recording(true);
                ui.set_mci_status(SharedString::from(msg));
            }
            Err(e) => ui.set_mci_status(SharedString::from(format!("❌ {}", e))),
        }
    });

    let ui_handle = ui.as_weak();
    let device = Arc::clone(&mci_device);
    ui.on_mci_save_recording(move || {
        let ui = ui_handle.unwrap();
        let save_path = ui.get_record_path().to_string();
        let mut dev = device.lock().unwrap();

        if !dev.is_recording {
            ui.set_mci_status(SharedString::from("⚠️ 没有正在进行的录音"));
            return;
        }

        match mci_save_recording("recorder", &save_path) {
            Ok(msg) => {
                dev.is_recording = false;
                ui.set_is_recording(false);
                ui.set_mci_status(SharedString::from(msg));
            }
            Err(e) => ui.set_mci_status(SharedString::from(format!("❌ {}", e))),
        }
    });

    ui.run()
}
