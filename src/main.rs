#![feature(iter_next_chunk)]
#![feature(array_zip)]
pub mod hamming;
pub mod types;

use std::{fs::File, io::{BufReader, Read, Write}, iter::repeat, path::{Path, PathBuf}};

use image::{RgbImage};
use types::GF2;

use crate::{hamming::encode::{get_syndrome, encode_in}, types::{byte_into_gf2, gf2_into_byte}};
// Первые 32 закодированные бита - длина файла
// 

fn encode(container_file: &str, payload_file: &str, output_file: &str)
{
    let image_container = image::open(container_file).expect("Image expected").to_rgb8();
    let mut reader = BufReader::new(File::open(payload_file).expect("File ./payload.txt expected"));
    let mut input_file = Vec::new();
    reader.read_to_end(&mut input_file).expect("Failed to read file");
    //Биты пикселей исходного изображения
    let mut container_stream = image_container.pixels().flat_map(|x|x.0);
    //Вместимость в битах
    let cap = image_container.width() * image_container.height() * 3 * 3 / 7 - 32;
    println!("Max capacity: {} bits ({} bytes) ", cap, cap / 8);

    if input_file.len() >= (cap / 8) as usize {
        let cap = cap / 8;
        let size = input_file.len();
        println!("File \"{payload_file}\" is too big for container file \"{container_file}\"({size} bytes/{cap} bytes)");
        panic!();
    }
    //Байты, что будут закодированы в начале контейнера для определения длины содержимого
    let filesize:[u8; 4] = (input_file.len() as u32).to_le_bytes();
    //Поток битов, что будет закодирован в изображении
    let mut bits = filesize.into_iter().chain(input_file.into_iter()).chain(repeat(0)).flat_map(byte_into_gf2);
    let mut output : Vec<u8> = vec![];

    loop {
        //Берём 7 байт из потока пикселей
        match container_stream.next_chunk::<7>() {
            Ok(chunk) => 
            {
                //Получаем самые незначительные биты пикселелей из потока пикселей
                let original_bits = chunk.map(|x|byte_into_gf2(x)[0]);
                
                let symptom = bits.next_chunk::<3>().expect("Endless stream is not endless. Very sus");

                let encoded_bits = encode_in(&original_bits, &symptom);

                let t = chunk.zip(encoded_bits).map(|x| 
                    {
                        let mut n = byte_into_gf2(x.0);
                        n[0] = x.1;
                        gf2_into_byte(&n)
                    }
                );
                output.extend(t);
            }
            Err(remainder) => 
            {
                output.extend(remainder);
                break;
            }
        }
    }
    let output_image = RgbImage::from_vec(image_container.width(), image_container.height(), output).expect("This vector can't be too small!");
    output_image.save_with_format(output_file, image::ImageFormat::Png).expect("Saving image must be successful")
}

fn decode(container_file: &str, output_file: &str, guess_filetype: bool)
{
    let image_container = image::open(container_file).expect("Image expected").to_rgb8();
    let container_stream = image_container.pixels().flat_map(|x|x.0);

    
    let mut bitstream = container_stream.map(|x| byte_into_gf2(x)[0]);
    //Не загружайте СЛИШКОМ большие файлы🥺
    let mut output_vec = vec![];
    while let Ok(chunk) = bitstream.next_chunk::<7>() {
        let bits = get_syndrome(&chunk);
        output_vec.extend(bits);
    }
    
    let mut output: Vec<_> = output_vec.chunks_exact(8).map(|x| 
        {
            let b: [GF2; 8] = x.try_into().expect("Chunk with size of 8 is not size of 8!");
            gf2_into_byte(&b)
        }).collect();
    let filesize:Vec<u8> = output.iter().take(4).copied().collect();
    let filesize = u32::from_le_bytes(filesize.try_into().unwrap());
    println!("Size of encoded file is {}", filesize);

    let cap = image_container.width() * image_container.height() * 3 * 3 / 7 - 32;

    if cap / 8 < filesize 
    {
        println!("Reported file size inside \"{container_file}\" bigger that potential container capacity. Probably container is empty or corrupted")
    }
    output.truncate((4 + filesize).try_into().unwrap());

    let mut output_file = output_file.to_owned();
    if guess_filetype
    {
        match infer::get(&output[4..]) {
            Some(f) =>  
            {
                println!("Filetype is {}, extension is {}", f.mime_type(), f.extension());
                let p = PathBuf::from(output_file);
                output_file = p.with_extension(f.extension()).to_str().unwrap().to_owned();
            },
            None => println!("Failed to guess filetype!"),
        } 
    }
    let mut f = File::create(output_file).expect("File is not created!");
    f.write_all(&output[4..]).expect("File write must be successful");
}

use clap::{Command};

fn main() {
    let matches = Command::new("image_hamming")
        .subcommand_required(true)
        .subcommand(
            clap::command!("encode").about("Encodes file-payload inside input image").arg(
                clap::arg!(<input>).required(true)
            ).arg(
                clap::arg!(<payload>).required(true)
            ).arg(
                clap::arg!([output]).required(false).default_missing_value("output.png")
            )
        )
        .subcommand(
            clap::command!("decode").about("Decodes file-payload from input image").arg(
                clap::arg!(<input>).required(true)
            ).arg(
                clap::arg!([output]).required(false).default_missing_value("payload")
            )
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("encode") {
        encode(matches.get_one::<String>("input").unwrap(), 
        matches.get_one::<String>("payload").unwrap(), 
        matches.get_one::<String>("output").unwrap_or(&("output.png".into())));
    }
    else if let Some(matches) = matches.subcommand_matches("decode") {
        decode(matches.get_one::<String>("input").unwrap(), 
        matches.get_one::<String>("output").unwrap_or(&("payload".into())), true);
    }
}
