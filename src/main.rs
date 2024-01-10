use clap::Parser;
use image::DynamicImage;
use image::{io::Reader as ImageReader,EncodableLayout};
use std::path::PathBuf;
use std::{fs,fs::File};
use std::io::Write;

#[derive(Parser)]
struct Cli{
    source_path:PathBuf,
    destination_path:PathBuf,
}
fn main() {
    let args = Cli::parse();
    let extension = match args.destination_path.extension(){
        Some(extension) => extension,
        None => panic!("Cannot identified image format"),
    }.to_str().unwrap();
    match extension.to_lowercase().as_str(){
        "png" => to_png_image(&args.source_path,&args.destination_path),
        "webp" => to_webp_image(&args.source_path,&args.destination_path),
        "jpg" => to_jpg_image(&args.source_path,&args.destination_path),
        _ => panic!("Invalid format"),
    }
}


fn to_png_image(path:&PathBuf,des:&PathBuf){
    let img = match image::open(path){
        Ok(img_reader) => img_reader,
        Err(e) => panic!("Problem processing image : {}",e),
    };
    
    let folder_path= match des.to_str(){
        Some(folder_string) => folder_string,
        None => panic!("Problem processing image"),
    };
    let file_name = match path.file_stem(){
        Some(name) => name,
        None => panic!("Problem processing image"),
    };
    let mut des_clone = des.clone();
    let file_path = match file_name.to_str(){
        Some(path) => {des_clone.push(format!("{}.png",path));des_clone},
        None => panic!("Problem processing image :"),
    };
    let _ = fs::create_dir_all(folder_path).unwrap();
    img.save(file_path).unwrap();
    success_message(file_name_from_Path(path), file_name_from_Path(des))
}

fn to_jpg_image(path:&PathBuf,des:&PathBuf){
    let img = match image::open(path){
        Ok(img_reader) => img_reader,
        Err(e) => panic!("Problem processing image : {}",e),
    };
    
    let folder_path= match des.to_str(){
        Some(folder_string) => folder_string,
        None => panic!("Problem processing image :"),
    };
    let file_name = match path.file_stem(){
        Some(name) => name,
        None => panic!("Problem processing image :"),
    };
    let mut des_clone = des.clone();
    let file_path = match file_name.to_str(){
        Some(path) => {des_clone.push(format!("{}.jpg",path));des_clone},
        None => panic!("Problem processing image :"),
    };
    let _ = fs::create_dir_all(folder_path).unwrap();
    img.save(file_path).unwrap();
}


fn to_webp_image(path:&PathBuf,des:&PathBuf){
    let img = match ImageReader::open(path){
        Ok(img_reader) => img_reader,
        Err(e) => panic!("Problem processing image : {}",e),
    };
    let decoded_img: DynamicImage = match img.decode() {
        Ok(decoded) => decoded,
        Err(e) => panic!("Problem processing image : {}",e),
    };
    let webp_memory: webp::WebPMemory = webp::Encoder::from_image(&decoded_img).unwrap().encode(100.0);
    let folder_path= match des.to_str(){
        Some(folder_string) => folder_string,
        None => panic!("Problem processing image :"),
    };
    let file_name = match path.file_stem(){
        Some(name) => name,
        None => panic!("Problem processing image :"),
    };
    let mut des_clone = des.clone();
    let file_path = match file_name.to_str(){
        Some(path) => {des_clone.push(format!("{}.webp",path));des_clone},
        None => panic!("Problem processing image :"),
    };
    let _ = fs::create_dir_all(folder_path).unwrap();
    let mut webp_image: File = File::create(file_path).unwrap();
    webp_image.write_all(webp_memory.as_bytes()).unwrap();
}

fn success_message(source_name:&str,converted_name:&str){
    println!("Successfully converted source image \"{}\" to \"{}\"",source_name,converted_name);
}

fn file_name_from_Path(path:&PathBuf)-> &str{
    let file_name_os = match path.file_name(){
        Some(name) => name,
        None => panic!("Unable to get file name from given path"),
    };
    let file_name = match file_name_os.to_str(){
        Some(file_name)=>file_name,
        None=> panic!("Unable to get file name from given path"),
    };
    return file_name;
}
