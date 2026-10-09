
use std::collections::btree_set::Range;
use std::{fs::File, result};

use std::io::{Read, Result, Write};

use image::metadata::CicpMatrixCoefficients::YCgCo;
struct form {
    width: u32,
    height: u32,
    Y: Vec<u8>,
    Cb: Vec<u8>,
    Cr: Vec<u8>
}
impl form {
    fn read(path: &str) -> Result<form>{
        let mut fl = File::open(path)?;
        let mut buf = [0 as u8;4];
        fl.read_exact(&mut buf)?;
        let w = u32::from_le_bytes(buf);
        fl.read_exact(&mut buf)?;
        let h = u32::from_le_bytes(buf);
        let size = (w * h) as usize;
        let mut Y = vec![0 as u8; size];
        fl.read_exact(&mut Y)?;
        let mut Cb = vec![0 as u8; size/2];
        fl.read_exact(&mut Cb)?;
        let mut Cr = vec![0 as u8; size/2];
        fl.read_exact(&mut Cr)?;
        return Ok(form { width: w as u32, height: h as u32, Y, Cb, Cr });
    }
    fn write(&self, path: &str )->std::io::Result<()>{
        let mut fl = File::create(path)?;
        fl.write_all(&self.width.to_le_bytes());
        fl.write_all(&self.height.to_le_bytes());
        fl.write_all(&self.Y)?;
        fl.write_all(&self.Cb)?;
        fl.write_all(&self.Cr)?;
        return Ok(());
    }
} 

struct rle_form {
    width: u32,
    height: u32,
    Y: Vec<(u8, u32)>,
    Cb: Vec<u8>
}

impl rle_form {
    fn write(&self, path: &str) -> std::io::Result<()> {
        let mut fl = File::create(path)?;
        fl.write_all(&self.width.to_le_bytes())?;
        fl.write_all(&self.height.to_le_bytes())?;
        let y_len = self.Y.len() as u32;
        fl.write_all(&y_len.to_le_bytes())?;

        for (value, count) in &self.Y {
            fl.write_all(&[*value])?;
            fl.write_all(&count.to_le_bytes())?;
        }
        fl.write_all(&self.Cb)?;

        return Ok(());
    }

    fn read(path: &str) -> Result<rle_form> {
        let mut fl = File::open(path)?;
        let mut buf4 = [0u8; 4];
        fl.read_exact(&mut buf4)?;
        let width = u32::from_le_bytes(buf4);
        fl.read_exact(&mut buf4)?;
        let height = u32::from_le_bytes(buf4);

        fl.read_exact(&mut buf4)?;
        let y_len = u32::from_le_bytes(buf4);
        let mut Y = Vec::with_capacity(y_len as usize);

        for _ in 0..y_len {
            let mut value = [0u8; 1];
            fl.read_exact(&mut value)?;

            fl.read_exact(&mut buf4)?;
            let count = u32::from_le_bytes(buf4);

            Y.push((value[0], count));
        }
        let cb_size = (width * height / 2) as usize;
        let mut Cb = vec![0u8; cb_size];
        fl.read_exact(&mut Cb)?;

        return Ok(rle_form {
            width,
            height,
            Y,
            Cb
        });
    }
}
fn clamp(n: i32)-> u8{
    if n > 255{
        return 255;
    } else if n < 0 {
        return 0;
    } else {
        return n as u8;
    }
    
}
fn to_ycbcr(channels :[u8; 3])-> [u8; 3]{
    let r =  channels[0] as f32;
    let g = channels[1] as f32;
    let b = channels[2] as f32;
    let Y = (0.299 * r) + (0.587 * g) + (0.114 * b);
    let Cb = 128.0 - (0.168736 * r) -(0.331264 *g) + (0.5 * b);
    let Cr = 128.0 + (0.5 * r) - (0.418688 * g) - (0.081312 *b);
    let result = [clamp(Y.round() as i32), clamp(Cb.round() as i32), clamp(Cr.round() as i32)];
    return result;
}
fn to_rgb(channels :[u8; 3]) -> [u8; 3]{
    let Y = channels[0] as f32;
    let Cb = channels[1] as f32;
    let Cr = channels[2] as f32;
    let r = Y + 1.402*(Cr-128.0);
    let g = Y - 0.34414 * (Cb - 128.0) - 0.71414*(Cr -128.0);
    let b = Y + 1.772*(Cb-128.0);
    let result = [clamp(r.round() as i32), clamp(g.round() as i32), clamp(b.round() as i32)];
    return result;
}
pub fn save4x4x4(){
    let mut img  = image::open("hrse.jpg").unwrap().to_rgb8();

    let w  =img.width();
    let h  = img.height();
    let mut Y: Vec<u8> = Vec::new();
    let mut Cb: Vec<u8> = Vec::new();
    let mut Cr: Vec<u8> = Vec::new();
    for i in 0..h{
        for j in 0..w{
            let p = img.get_pixel(j, i).0;
            let new_p = to_ycbcr(p);
            Y.push(new_p[0]);
            Cb.push(new_p[1]);
            Cr.push(new_p[2]);
        }
    }
    let res = form{
        width: w,
        height: h,
        Y,
        Cb,
        Cr
    };
    res.write("result4:4:4");
}
pub fn save4x2x2(){
    let mut img  = image::open("hrse.jpg").unwrap().to_rgb8();
    // let mut file = File::create("4x4x4.form");
    let w  =img.width();
    let h  = img.height();
    let mut Y: Vec<u8> = Vec::new();
    let mut Cb: Vec<u8> = Vec::new();
    let mut Cr: Vec<u8> = Vec::new();
    for i in 0..h{
        for j in 0..w{
            let p = img.get_pixel(j, i).0;
            let new_p = to_ycbcr(p);
            Y.push(new_p[0]);
            if j % 2 ==0{
                Cb.push(new_p[1]);
                Cr.push(new_p[2]);
            }
            
        }
    }
    let res = form{
        width: w,
        height: h,
        Y,
        Cb,
        Cr
    };
    res.write("result4:2:2");
}
fn to_rle(Y: Vec<u8>) -> Vec<(u8,u32)>{
    let mut rle_Y: Vec<(u8,u32)> = Vec::new();
    let mut pivot: (u8, u32) = (Y[0], 1);
    for i in 1..Y.len(){
        if Y[i] == pivot.0 {
            pivot.1 +=1;
        } else {
            rle_Y.push(pivot);
            pivot = (Y[i],1);
        }
    }
    return rle_Y;
}
fn from_rle(rle_Y :Vec<(u8,u32)>)-> Vec<u8>{
    let mut Y:Vec<u8> = Vec::new();
    for ch in &rle_Y{
        Y.extend(std::iter::repeat(ch.0).take(ch.1 as usize));
    }

    return Y;
}
pub fn save4x2x0(){
    let mut img  = image::open("hrse.jpg").unwrap().to_rgb8(); 
    let w  =img.width();
    let h  = img.height();
    let mut Y: Vec<u8> = Vec::new();
    let mut Cb: Vec<u8> = Vec::new();
    let mut Cr: Vec<u8> = Vec::new();
    for i in 0..h{
        for j in 0..w{
            let p = img.get_pixel(j, i).0;
            let new_p = to_ycbcr(p);
            Y.push(new_p[0]);
            Cb.push(new_p[1]);
            Cr.push(new_p[2]);
              
        }
    }
    let mut C: Vec<u8> = Vec::new();
    for i in (0..Cb.len()).step_by(2){
        let ch1 = Cb[i] as i32;
        let ch2 = Cb[i+1] as i32;
        let ch3 = Cr[i] as i32;
        let ch4 = Cr[i+1] as i32;
        let result = ((ch1 + ch2 + ch3 + ch4) /4) as u8;
        C.push(result);
    }
    let Y = to_rle(Y);
    let res = rle_form{
        width: w,
        height: h,
        Y,
        Cb:C
    };
    res.write("result4:2:0");
}
pub fn load4x4x4(){
    let ycbcr_img = form::read("result4:4:4").unwrap();
    let w = ycbcr_img.width;
    let h = ycbcr_img.height;
    let Y = ycbcr_img.Y;
    let Cb = ycbcr_img.Cb;
    let Cr = ycbcr_img.Cr;
    let mut img = image::RgbImage::new(w, h);
    for i in 0..h{
        for j in 0..w{
            
           let p = [Y[(j + i*w)as usize] , Cb[(j + i*w)as usize], Cr[(j+i*w)as usize]];
           let new_p = to_rgb(p);
           img.put_pixel(j, i, image::Rgb(new_p)); 
              
        }
    }
    img.save("from4x4x4.png");
}
pub fn load4x2x2(){
    let ycbcr_img = form::read("result4:2:2").unwrap();
    let w = ycbcr_img.width;
    let h = ycbcr_img.height;
    let Y = ycbcr_img.Y;
    let Cb = ycbcr_img.Cb;
    let Cr = ycbcr_img.Cr;
    let mut img = image::RgbImage::new(w, h);
    for i in 0..h{
        for j in 0..w{
            
           let p = [Y[(j + i*w)as usize] , Cb[((j + i*w)/2)as usize], Cr[((j+i*w)/2)as usize]];
           let new_p = to_rgb(p);
           img.put_pixel(j, i, image::Rgb(new_p)); 
              
        }
    }
    img.save("from4x2x2.png");
}
pub fn load4x2x0(){
    let ycbcr_img = rle_form::read("result4:2:0").unwrap();
    let w = ycbcr_img.width;
    let h = ycbcr_img.height;
    let Y = from_rle(ycbcr_img.Y);

    let Cb = ycbcr_img.Cb;

    let mut img = image::RgbImage::new(w, h);
    for i in (0..h-1).step_by(2){
        for j in (0..w-1).step_by(2){
            let Y1 = Y[(j + i*w)as usize];
            let Y2 = Y[((j+1) + i*w)as usize];
            let Y3 = Y[((j+1) + i*w+1)as usize];
            let Y4 = Y[(j + i*w+1)as usize];
            let Cr = Cb[((j + i*w)/2)as usize];

            let new_p1 = to_rgb([Y1, Cr, Cr]);
            let new_p2 = to_rgb([Y2, Cr, Cr]);
            let new_p3 = to_rgb([Y3, Cr, Cr]);
            let new_p4 = to_rgb([Y4, Cr, Cr]);
            
            img.put_pixel(j, i, image::Rgb(new_p1)); 
            img.put_pixel(j+1, i, image::Rgb(new_p2)); 
            img.put_pixel(j+1, i+1, image::Rgb(new_p3)); 
            img.put_pixel(j, i+1, image::Rgb(new_p4)); 
              
        }
    }
    img.save("from4x2x0.png");
}
