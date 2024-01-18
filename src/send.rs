use crate::error::{from_rusb_error, Error, Result};
use rusb::{DeviceHandle, GlobalContext};
use std::time::Duration;

pub fn send(source: &str) -> Result<()> {
    let code = encode(source)?;
    let handle = get_handle()?;
    set_code(&handle, code).and_then(|_| send_req(&handle))?;

    Ok(())
}

fn encode(source: &str) -> Result<Vec<u8>> {
    let leader: Vec<u8> = vec![0x00, 0x88, 0x00, 0x44];
    let trailer: Vec<u8> = vec![0x00, 0x11, 0x0b, 0x1c];

    let parsed: Vec<u8> = source
        .chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).map_err(|_| Error::Parse))
        .collect::<Result<_>>()?;

    let code: Vec<u8> = parsed
        .into_iter()
        .map(|digit| !digit)
        .flat_map(|digit| {
            (0..4).rev().flat_map(move |index| {
                let bit = (digit >> index) & 1;
                if bit == 1 {
                    // on
                    [0x00, 0x11, 0x00, 0x11]
                } else {
                    // off
                    [0x00, 0x11, 0x00, 0x33]
                }
            })
        })
        .collect();

    Ok([leader, code, trailer].concat())
}

fn get_handle() -> Result<DeviceHandle<GlobalContext>> {
    let vid: u16 = 0x22ea;
    let pid: u16 = 0x003a;

    let devices = rusb::devices().map_err(|e| from_rusb_error(e))?;
    let device = devices
        .iter()
        .find(|device| {
            let res = device.device_descriptor().map(|desc| {
                let cond1 = desc.vendor_id() == vid;
                let cond2 = desc.product_id() == pid;
                cond1 && cond2
            });

            res == rusb::Result::Ok(true)
        })
        .ok_or(Error::NotFound)?;

    device.open().map_err(|e| from_rusb_error(e))
}

fn set_code(handle: &DeviceHandle<GlobalContext>, code: Vec<u8>) -> Result<()> {
    let code_len = code.len();
    let max_len_in_one = 56;

    let mut sent_len = 0;
    while sent_len < code_len {
        let remain_len = code_len - sent_len;
        let send_len = std::cmp::min(remain_len, max_len_in_one);

        let slice = &code[sent_len..(sent_len + send_len)];
        let slice: Vec<u8> = slice.iter().copied().collect();

        let byte0: u8 = 0x34;
        let byte1: u8 = (((code_len / 4) >> 8) & 0xff).try_into().unwrap();
        let byte2: u8 = ((code_len / 4) & 0xff).try_into().unwrap();
        let byte3: u8 = (((sent_len / 4) >> 8) & 0xff).try_into().unwrap();
        let byte4: u8 = ((sent_len / 4) & 0xff).try_into().unwrap();
        let byte5: u8 = ((send_len / 4) & 0xff).try_into().unwrap();
        let leader: Vec<u8> = vec![byte0, byte1, byte2, byte3, byte4, byte5];
        let trailer: Vec<u8> = vec![0x00, 0x00];

        let data = [leader, slice, trailer].concat();
        handle
            .write_interrupt(0x04, &data, Duration::from_millis(5000))
            .map_err(|e| from_rusb_error(e))?;

        sent_len += send_len;
    }

    Ok(())
}

fn send_req(handle: &DeviceHandle<GlobalContext>) -> Result<()> {
    let buf: [u8; 64] = [
        0x35, 0x94, 0x70, 0x00, 0x2a, 0x00, 0x00, 0x11, 0x00, 0x34, 0x00, 0x10, 0x00, 0x12, 0x00,
        0x11, 0x00, 0x33, 0x00, 0x11, 0x00, 0x12, 0x00, 0x11, 0x00, 0x12, 0x00, 0x10, 0x00, 0x13,
        0x00, 0x11, 0x00, 0x34, 0x00, 0x10, 0x00, 0x34, 0x00, 0x10, 0x00, 0x12, 0x00, 0x11, 0x00,
        0x12, 0x00, 0x11, 0x00, 0x34, 0x00, 0x10, 0x00, 0x12, 0x00, 0x11, 0x00, 0x12, 0x00, 0x11,
        0x0b, 0x1c, 0x00, 0x00,
    ];

    handle
        .write_interrupt(0x04, &buf, Duration::from_millis(5000))
        .map(|_| ())
        .map_err(|e| from_rusb_error(e))
}
