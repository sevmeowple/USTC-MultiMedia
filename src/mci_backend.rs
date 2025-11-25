use std::path::Path;
use windows::Win32::Media::Audio::{PlaySoundW, SND_ASYNC, SND_FILENAME, SND_LOOP, SND_SYNC};
use windows::Win32::Media::Multimedia::{mciGetErrorStringW, mciSendStringW};
use windows::core::PCWSTR;

// ============ PlaySound API ============

pub fn play_sound_sync(path: &str) -> Result<(), String> {
    if !Path::new(path).exists() {
        return Err(format!("文件不存在: {}", path));
    }
    let path_wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe { PlaySoundW(PCWSTR(path_wide.as_ptr()), None, SND_FILENAME | SND_SYNC) };
    Ok(())
}

pub fn play_sound_async(path: &str) -> Result<(), String> {
    if !Path::new(path).exists() {
        return Err(format!("文件不存在: {}", path));
    }
    let path_wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe { PlaySoundW(PCWSTR(path_wide.as_ptr()), None, SND_FILENAME | SND_ASYNC) };
    Ok(())
}

pub fn play_sound_loop(path: &str) -> Result<(), String> {
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

pub fn stop_sound() {
    unsafe {
        PlaySoundW(PCWSTR::null(), None, Default::default());
    }
}

// ============ MCI API ============

pub struct MCIDevice {
    pub alias: String,
    pub is_open: bool,
    pub is_recording: bool,
}

impl MCIDevice {
    pub fn new() -> Self {
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
        "waveaudio"
    }
}

pub fn mci_open_file(path: &str, alias: &str) -> Result<String, String> {
    if !Path::new(path).exists() {
        return Err(format!("文件不存在: {}", path));
    }
    let device_type = get_device_type(path);
    let command = format!("open \"{}\" type {} alias {}", path, device_type, alias);
    mci_send_string(&command)?;
    Ok(format!("已打开: {} ({})", path, device_type))
}

pub fn mci_play(alias: &str) -> Result<String, String> {
    let command = format!("play {}", alias);
    mci_send_string(&command)?;
    Ok("播放中...".to_string())
}

pub fn mci_pause(alias: &str) -> Result<String, String> {
    let command = format!("pause {}", alias);
    mci_send_string(&command)?;
    Ok("已暂停".to_string())
}

pub fn mci_resume(alias: &str) -> Result<String, String> {
    let command = format!("resume {}", alias);
    mci_send_string(&command)?;
    Ok("继续播放...".to_string())
}

pub fn mci_stop(alias: &str) -> Result<String, String> {
    let command = format!("stop {}", alias);
    mci_send_string(&command)?;
    Ok("已停止".to_string())
}

pub fn mci_close(alias: &str) -> Result<String, String> {
    let command = format!("close {}", alias);
    mci_send_string(&command)?;
    Ok("已关闭设备".to_string())
}

pub fn mci_record(alias: &str) -> Result<String, String> {
    let command = format!("open new type waveaudio alias {}", alias);
    mci_send_string(&command)?;
    let command = format!("record {}", alias);
    mci_send_string(&command)?;
    Ok("录音中...".to_string())
}

pub fn mci_save_recording(alias: &str, save_path: &str) -> Result<String, String> {
    let command = format!("stop {}", alias);
    mci_send_string(&command)?;
    let command = format!("save {} \"{}\"", alias, save_path);
    mci_send_string(&command)?;
    let command = format!("close {}", alias);
    mci_send_string(&command)?;
    Ok(format!("录音已保存: {}", save_path))
}