use std::mem;
use std::path::Path;
use std::thread;
use std::time::Duration;

// use windows::core::PCWSTR;
use windows::Win32::Media::Audio::{
    waveOutClose, waveOutOpen, waveOutPrepareHeader, waveOutUnprepareHeader, waveOutWrite,
    HWAVEOUT, WAVEFORMATEX, WAVEHDR, WAVE_MAPPER, WHDR_DONE, CALLBACK_NULL,
};
use windows::Win32::Media::Multimedia::{
    mmioAscend, mmioClose, mmioDescend, mmioOpenW, mmioRead, MMCKINFO, MMIO_FINDCHUNK,
    MMIO_FINDRIFF, MMIO_READ,
};

// 辅助函数:生成 FOURCC 代码
fn make_fourcc(c1: u8, c2: u8, c3: u8, c4: u8) -> u32 {
    (c1 as u32) | ((c2 as u32) << 8) | ((c3 as u32) << 16) | ((c4 as u32) << 24)
}

/// 使用 MMIO 读取 WAV 文件并使用 WaveOut 播放
/// 这是一个阻塞函数,建议在单独的线程中运行
pub fn play_wave_file_mmio(path: &str) -> Result<String, String> {
    if !Path::new(path).exists() {
        return Err(format!("文件不存在: {}", path));
    }

    let mut path_wide: Vec<u16> = path.encode_utf16().collect();
    path_wide.resize(128, 0); // 填充到 128 长度

    unsafe {
        // 1. 打开文件 (mmioOpen)
        let hmmio = mmioOpenW(Some(&mut path_wide.try_into().unwrap()), None, MMIO_READ);
        if hmmio.is_invalid() {
            return Err("无法打开文件 (mmioOpen 失败)".to_string());
        }

        // 2. 查找 RIFF/WAVE 块
        let mut chunk_riff = MMCKINFO::default();
        if mmioDescend(hmmio, &mut chunk_riff, None, MMIO_FINDRIFF) != 0 {
            mmioClose(hmmio, 0);
            return Err("不是有效的 RIFF 文件".to_string());
        }

        if chunk_riff.fccType != make_fourcc(b'W', b'A', b'V', b'E') {
            mmioClose(hmmio, 0);
            return Err("不是有效的 WAVE 文件".to_string());
        }

        // 3. 查找 fmt 块
        let mut chunk_fmt = MMCKINFO::default();
        chunk_fmt.ckid = make_fourcc(b'f', b'm', b't', b' ');
        if mmioDescend(hmmio, &mut chunk_fmt, Some(&chunk_riff), MMIO_FINDCHUNK) != 0 {
            mmioClose(hmmio, 0);
            return Err("找不到 fmt 块".to_string());
        }

        // 读取格式信息
        let mut fmt_bytes = vec![0u8; chunk_fmt.cksize as usize];
        let size_read = mmioRead(hmmio, &mut fmt_bytes);
        
        let mut wave_format = WAVEFORMATEX::default();
        if size_read > 0 {
            std::ptr::copy_nonoverlapping(
                fmt_bytes.as_ptr(),
                &mut wave_format as *mut _ as *mut u8,
                std::cmp::min(size_read as usize, std::mem::size_of::<WAVEFORMATEX>())
            );
        }

        // 跳出 fmt 块
        mmioAscend(hmmio, &mut chunk_fmt, 0);

        // 4. 查找 data 块
        let mut chunk_data = MMCKINFO::default();
        chunk_data.ckid = make_fourcc(b'd', b'a', b't', b'a');
        if mmioDescend(hmmio, &mut chunk_data, Some(&chunk_riff), MMIO_FINDCHUNK) != 0 {
            mmioClose(hmmio, 0);
            return Err("找不到 data 块".to_string());
        }
        
        let data_size = chunk_data.cksize;
        
        // 分配内存并读取音频数据
        let mut data_buffer = vec![0u8; data_size as usize];
        let read_bytes = mmioRead(hmmio, &mut data_buffer);

        mmioClose(hmmio, 0); // 文件读取完毕,关闭文件

        if read_bytes != data_size as i32 {
            return Err("读取音频数据失败".to_string());
        }

        // 5. 打开音频设备 (WaveOutOpen)
        let mut h_waveout = HWAVEOUT::default();
        let mm_res = waveOutOpen(
            Some(&mut h_waveout),
            WAVE_MAPPER,
            &wave_format,
            None,
            None,
            CALLBACK_NULL,
        );

        if mm_res != 0 {
            return Err("无法打开音频设备 (waveOutOpen 失败)".to_string());
        }

        // 6. 准备 Header
        let mut wave_hdr = WAVEHDR {
            lpData: windows::core::PSTR(data_buffer.as_mut_ptr()),
            dwBufferLength: data_size,
            dwBytesRecorded: 0,
            dwUser: 0,
            dwFlags: 0,
            dwLoops: 0,
            lpNext: std::ptr::null_mut(),
            reserved: 0,
        };

        if waveOutPrepareHeader(h_waveout, &mut wave_hdr, mem::size_of::<WAVEHDR>() as u32) != 0 {
            waveOutClose(h_waveout);
            return Err("waveOutPrepareHeader 失败".to_string());
        }

        // 7. 写入数据开始播放
        if waveOutWrite(h_waveout, &mut wave_hdr, mem::size_of::<WAVEHDR>() as u32) != 0 {
            waveOutUnprepareHeader(h_waveout, &mut wave_hdr, mem::size_of::<WAVEHDR>() as u32);
            waveOutClose(h_waveout);
            return Err("waveOutWrite 失败".to_string());
        }

        // 8. 等待播放完成
        while (wave_hdr.dwFlags & WHDR_DONE) == 0 {
            thread::sleep(Duration::from_millis(50));
        }

        // 9. 清理资源
        waveOutUnprepareHeader(h_waveout, &mut wave_hdr, mem::size_of::<WAVEHDR>() as u32);
        waveOutClose(h_waveout);

        let sample_rate = wave_format.nSamplesPerSec;
        Ok(format!(
            "播放完成 (采样率: {}Hz, 大小: {}KB)",
            sample_rate,
            data_size / 1024
        ))
    }
}