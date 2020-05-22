# rust_qrcode_excel

> Tools for mass production of QR codes
> 
> urls with parameters to generate QR code from excel file

Easy to use:


Install Rust. rustup.rs heavily recommended. See https://www.rustup.rs/ for details

Alternatively, you can also use multirust. See https://github.com/brson/multirust for details

```
$ cargo run FILENAME[xlsx/xls]
```

Example:
```
cd generate_qrcode/
cargo run data_demo.xlsx 
```
Result 

>The QR code will be saved in the project folder