use spectrum_analyzer::scaling::divide_by_N;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
pub const PACKAGE_LEN: usize = 10934;
pub const PAD_TO_LEN: usize = 4096;
pub struct SerialParameter {
    pub port_name: String,
    pub baud_rate: i32,
}
pub fn collect_fft(buf: &[u8]) -> Vec<Vec<f32>> {
    let mut iter = buf.iter().enumerate();
    // let mut iter = iter.skip(70);
    let mut index;
    loop {
        let (_indx, current) = iter.next().unwrap();
        print!("{:02x} ", current);
        if *current == 0x00 {
            let (indx, current) = iter.next().unwrap();
            if *current == 0x00 {
                if buf[indx + 1] == 0x00 {
                    if buf[indx + 2] == 0x00 {
                        index = indx + 3;
                    } else {
                        index = indx + 2;
                    }
                } else {
                    index = indx + 1;
                }
                println!("found:{index}");
                break;
            }
        }
    }
    let mut init_index = 0;
    let mut nums = Vec::with_capacity(2048);
    loop {
        if index > PACKAGE_LEN - 2 {
            break;
        }
        let number = ((buf[index + 1] as u16) << 8) | buf[index] as u16;
        if (init_index + 1) % 4 == 0 && number != 0 {
            println!("{index}");
            println!("{:?}", &buf[0..index + 10]);
            break;
        }
        index += 2;
        init_index += 1;
        nums.push(number as f32);
    }
    let xyz_nums_4chunk = nums[..(nums.len() / 4 * 4)].chunks(4);

    let mut fft_batch: Vec<Vec<f32>> = (0..3)
        .map(|indx| {
            xyz_nums_4chunk
                .clone()
                .map(|item| item[indx] * 60.0 / 1023.0 - 30.0)
                .collect()
        })
        .collect();

    let xyz_sqrt = fft_batch[0]
        .iter()
        .zip(fft_batch[1].iter())
        .zip(fft_batch[2].iter())
        .map(|((x, y), z)| f32::sqrt(x * x + y * y + z * z))
        .collect();
    fft_batch.push(xyz_sqrt);

    fft_batch
        .iter_mut()
        .for_each(|item| item.resize(PAD_TO_LEN, 0.0));

    let xyz_spec: Vec<Vec<f32>> = fft_batch
        .iter()
        .map(|xyz| {
            let spectrum =
                samples_fft_to_spectrum(&xyz, 1440, FrequencyLimit::Max(100.0), Some(&divide_by_N))
                    .unwrap();
            spectrum
                .data()
                .iter()
                .take(128)
                .map(|(_f, v)| (*v).val())
                .collect()
        })
        .collect();
    xyz_spec
}
