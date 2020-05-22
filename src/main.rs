use std::env;
use std::io::Write;
use std::io::Read;
use std::process::Command;
use std::process;

use qrcode::{QrCode};
use qrcode::render::svg;
use calamine::{Reader, Xlsx, open_workbook};


fn main(){

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let f_name = &args.get(1).expect("请检查命令是否符合要求");
        let file_suiffx = &f_name.split(".").collect::<Vec<&str>>();
        if file_suiffx.len() != 2 {
            eprintln!("你输的是什么JB文件名");
        }else{
            let check_file_vec = ["xlsx","xls"];
        
            if check_file_vec.contains(&file_suiffx[1]){
                let data = read_xls(f_name.to_string());
                for data in &data[1..] {
                    gen_qrcode(data.to_string());
                }
            }else{
                eprintln!("文件格式不正确，请检查命令是否符合要求")
            };
        }
    }else{
        eprintln!("You have to enter a wrong filename");
        process::exit(1);
    }


    
    // println!("{:?}", &args);
}

fn read_xls(fname: String) -> Vec<String>{
    
    let mut excel: Xlsx<_> = open_workbook(&fname).unwrap();
    let mut url = String::new();
    let mut data = Vec::new();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
       
        for row in r.rows() {
            url = row[0].to_string() + "&" + &row[1].to_string() + "&" + &row[2].to_string() + "&" + &row[3].to_string() + "&" + &row[4].to_string()+ "&" + &row[5].to_string();
            data.push(url);
        }
    };
    data
}


fn gen_qrcode(url: String) {

    let code = QrCode::new(&url).unwrap();

    let image = code.render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#E26E42"))
        .build();
    
    println!("{}", url);
    let mut write_svg = std::fs::File::create(url[15..].to_string().to_owned() + ".svg").unwrap();
    write_svg.write_all(image.as_bytes()).unwrap();
}


