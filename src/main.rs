mod mci_backend;
mod mmio_backend;

use slint::ComponentHandle;
use std::thread;

slint::include_modules!();

fn main() {
    let ui = AppWindow::new().unwrap();

    // MMIO 播放
    let ui_weak = ui.as_weak();
    ui.on_mmio_play_file(move |path| {
        let ui = ui_weak.unwrap();
        let path_str = path.to_string();
        ui.set_mmio_status("正在播放...".into());
        
        let ui_weak2 = ui.as_weak();
        thread::spawn(move || {
            let result = mmio_backend::play_wave_file_mmio(&path_str);
            let ui = ui_weak2.unwrap();
            match result {
                Ok(msg) => ui.set_mmio_status(msg.into()),
                Err(e) => ui.set_mmio_status(format!("错误: {}", e).into()),
            }
        });
    });

    // PlaySound 同步
    let ui_weak = ui.as_weak();
    ui.on_play_sound_sync(move |path| {
        let ui = ui_weak.unwrap();
        let path_str = path.to_string();
        ui.set_mmio_status("同步播放中...".into());
        
        let ui_weak2 = ui.as_weak();
        thread::spawn(move || {
            let result = mci_backend::play_sound_sync(&path_str);
            let ui = ui_weak2.unwrap();
            match result {
                Ok(_) => ui.set_mmio_status("播放完成".into()),
                Err(e) => ui.set_mmio_status(format!("错误: {}", e).into()),
            }
        });
    });

    // PlaySound 异步
    let ui_weak = ui.as_weak();
    ui.on_play_sound_async(move |path| {
        let ui = ui_weak.unwrap();
        match mci_backend::play_sound_async(&path.to_string()) {
            Ok(_) => ui.set_mmio_status("异步播放已启动".into()),
            Err(e) => ui.set_mmio_status(format!("错误: {}", e).into()),
        }
    });

    // PlaySound 循环
    let ui_weak = ui.as_weak();
    ui.on_play_sound_loop(move |path| {
        let ui = ui_weak.unwrap();
        match mci_backend::play_sound_loop(&path.to_string()) {
            Ok(_) => ui.set_mmio_status("循环播放已启动".into()),
            Err(e) => ui.set_mmio_status(format!("错误: {}", e).into()),
        }
    });

    // 停止声音
    let ui_weak = ui.as_weak();
    ui.on_stop_sound(move || {
        let ui = ui_weak.unwrap();
        mci_backend::stop_sound();
        ui.set_mmio_status("已停止".into());
    });

    // MCI 打开文件
    let ui_weak = ui.as_weak();
    ui.on_mci_open_file(move |path| {
        let ui = ui_weak.unwrap();
        match mci_backend::mci_open_file(&path.to_string(), "media_device") {
            Ok(msg) => {
                ui.set_mci_status(msg.into());
                ui.set_mci_is_open(true);
            }
            Err(e) => ui.set_mci_status(format!("错误: {}", e).into()),
        }
    });

    // MCI 播放
    let ui_weak = ui.as_weak();
    ui.on_mci_play(move || {
        let ui = ui_weak.unwrap();
        match mci_backend::mci_play("media_device") {
            Ok(msg) => ui.set_mci_status(msg.into()),
            Err(e) => ui.set_mci_status(format!("错误: {}", e).into()),
        }
    });

    // MCI 暂停
    let ui_weak = ui.as_weak();
    ui.on_mci_pause(move || {
        let ui = ui_weak.unwrap();
        match mci_backend::mci_pause("media_device") {
            Ok(msg) => ui.set_mci_status(msg.into()),
            Err(e) => ui.set_mci_status(format!("错误: {}", e).into()),
        }
    });

    // MCI 继续
    let ui_weak = ui.as_weak();
    ui.on_mci_resume(move || {
        let ui = ui_weak.unwrap();
        match mci_backend::mci_resume("media_device") {
            Ok(msg) => ui.set_mci_status(msg.into()),
            Err(e) => ui.set_mci_status(format!("错误: {}", e).into()),
        }
    });

    // MCI 停止
    let ui_weak = ui.as_weak();
    ui.on_mci_stop(move || {
        let ui = ui_weak.unwrap();
        match mci_backend::mci_stop("media_device") {
            Ok(msg) => ui.set_mci_status(msg.into()),
            Err(e) => ui.set_mci_status(format!("错误: {}", e).into()),
        }
    });

    // MCI 关闭
    let ui_weak = ui.as_weak();
    ui.on_mci_close(move || {
        let ui = ui_weak.unwrap();
        match mci_backend::mci_close("media_device") {
            Ok(msg) => {
                ui.set_mci_status(msg.into());
                ui.set_mci_is_open(false);
            }
            Err(e) => ui.set_mci_status(format!("错误: {}", e).into()),
        }
    });

    // MCI 录音
    let ui_weak = ui.as_weak();
    ui.on_mci_record(move || {
        let ui = ui_weak.unwrap();
        match mci_backend::mci_record("recorder") {
            Ok(msg) => {
                ui.set_mci_status(msg.into());
                ui.set_mci_is_recording(true);
            }
            Err(e) => ui.set_mci_status(format!("错误: {}", e).into()),
        }
    });

    // MCI 保存录音
    let ui_weak = ui.as_weak();
    ui.on_mci_save_recording(move |path| {
        let ui = ui_weak.unwrap();
        match mci_backend::mci_save_recording("recorder", &path.to_string()) {
            Ok(msg) => {
                ui.set_mci_status(msg.into());
                ui.set_mci_is_recording(false);
            }
            Err(e) => ui.set_mci_status(format!("错误: {}", e).into()),
        }
    });

    ui.run().unwrap();
}