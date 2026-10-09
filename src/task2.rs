use core::range;



fn clamp(n :i32) -> u8{
    if n > 255 {
        return 255;
    } else if n < 0 {
        return 0;
    }
     else {
        return n as u8;
    }
}
pub fn bright(n :u8){
    let mut img  = image::open("hrse.jpg").unwrap().to_rgb8();
    for i in 0..img.height(){
        for j in 0..img.width(){
            let p = img.get_pixel(j, i).0;
            let mut new_p = [0,0,0];
            for k in 0..3 {
                new_p[k] = clamp(p[k] as i32 + n as i32);
            }
            img.put_pixel(j, i, image::Rgb(new_p));
        }
    }
    img.save("bright.png");
}
pub fn contrast(n: u8){
    let mut img  = image::open("hrse.jpg").unwrap().to_rgb8();
    for i in 0..img.height(){
        for j in 0..img.width(){
            let p = img.get_pixel(j, i).0;
            let mut new_p = [0,0,0];
            for k in 0..3 {
                new_p[k] = clamp(128 + n as i32 * (p[k] as i32 - 128));
            }
            img.put_pixel(j, i, image::Rgb(new_p));
        }
    }
    img.save("contrast.png");

}
pub fn add_rgb(channels : [u8;3]){
    let mut img  = image::open("hrse.jpg").unwrap().to_rgb8();
    for i in 0..img.height(){
        for j in 0..img.width(){
            let p = img.get_pixel(j, i).0;
            let mut new_p = [0,0,0];
            for k in 0..3 {
                new_p[k] = clamp(p[k] as i32 + channels[k] as i32);
            }
            img.put_pixel(j, i, image::Rgb(new_p));
        }
    }
    img.save("output2.2.png"); 
}

fn to_rgb(channels : [f32;3])-> [u8; 3]{
    let C = channels[1]*channels[2];
    let X = C*(1.0-((channels[0]/60.0)% 2.0 -1.0).abs());
    let m = channels[2] -C;
    let res: [f32;3];
    let H = channels[0] as i32;
    let mut r1= 0.0;
    let mut r2= 0.0;
    let mut r3= 0.0;
    if (0 <= H && H < 60) {
        r1 = C;
        r2 = X;
        r3 = 0.0;
    } else if (60 <= H && H < 120){
        r1 = X;
        r2 = C;
        r3 = 0.0;
    } else if (120 <= H && H < 180){
        r1 = 0.0;
        r2 = C;
        r3 = X;
    } else if (180 <= H && H < 240){
        r1 = 0.0;
        r2 = X;
        r3 = C;
    } else if (240 <= H && H <300){
        r1 = X;
        r2 = 0.0;
        r3 = C;
    } else if (300 <= H && H < 360){
        r1 = C;
        r2 = 0.0;
        r3 = X;
    }
    let res:[u8;3] = [clamp(((r1+m) *255.0) as i32) as u8,clamp(((r2+m)*255.0) as i32) as u8, clamp(((r3+m) *255.0) as i32)as u8];
    return  res;
}

fn to_hsv(channels: [u8;3])-> [f32; 3]{
    let r = channels[0] as f32 / 255.0;
    let g = channels[1] as f32 / 255.0;
    let b = channels[2] as f32 / 255.0;
    let norm_rgb:[f32; 3] = [r,g,b];
    let max = norm_rgb.iter().copied().max_by(|a, b| a.total_cmp(b)).unwrap();
    let min = norm_rgb.iter().copied().min_by(|a, b| a.total_cmp(b)).unwrap();
    let V = max;
    let mut S: f32;
    let mut H: f32 =0.0;
    let delta = max - min;
    if max == 0.0 {
        S =0.0;
    } else {
        S = delta/max;
    }
    if delta == 0.0{
        H = 0.0;
    } else if max == r{
        H = 60.0 * ((g - b) / delta);
    } else if max == g{
        H = 60.0 * ((b - r) / delta + 2.0);
    } else if max == b{
        H = 60.0 * ((r - g) / delta + 4.0);
    } 
    if H < 0.0{
         H += 360.0;
    }    
    return [H,S,V]; 
}
pub fn tone(angle :i16){
    let mut img  = image::open("hrse.jpg").unwrap().to_rgb8();
    for i in 0..img.height(){
        for j in 0..img.width(){
            let p = img.get_pixel(j, i).0;
            let mut hsv_p = to_hsv(p);
            let new_h = hsv_p[0] as i16 + angle;
            if new_h > 360 {
                let new_h = new_h - 360;
            } if new_h < 0 {
                let new_h = new_h + 360;
            }
            hsv_p[0] = new_h as f32;
            let p = to_rgb(hsv_p);
            img.put_pixel(j, i, image::Rgb((p))); 
        }
    }
    img.save("tone.png"); 
}
pub fn saturate(factor :f32){
    let mut img  = image::open("hrse.jpg").unwrap().to_rgb8();
    for i in 0..img.height(){
        for j in 0..img.width(){
            let p = img.get_pixel(j, i).0;
            let mut hsv_p = to_hsv(p);
            let new_s = hsv_p[2] * factor;
            hsv_p[0] = clamp_saturation(new_s);
            let p = to_rgb(hsv_p);
            img.put_pixel(j, i, image::Rgb((p))); 
        }
    }
    img.save("saturate.png"); 
}
fn clamp_saturation(n :f32) -> f32{
    if n > 1.0{
        return 1.0;
    } else if n < 0.0{
        return 0.0;
    } else{
        return n;
    }
}
pub fn negative(){
    let mut img  = image::open("hrse.jpg").unwrap().to_rgb8();
    for i in 0..img.height(){
        for j in 0..img.width(){
            let p = img.get_pixel(j, i).0;
            let mut new_p = [clamp(255-p[0] as i32),clamp(255-p[1] as i32),clamp(255-p[2] as i32)];
            
            img.put_pixel(j, i, image::Rgb(new_p));
        }
    }
    img.save("negative.png"); 
}
pub fn sepia(){
    let mut img  = image::open("hrse.jpg").unwrap().to_rgb8();
    for i in 0..img.height(){
        for j in 0..img.width(){
            let p = img.get_pixel(j, i).0;
            let R = 0.393 * p[0] as f32 + 0.769 *p[1] as f32 + 0.189 * p[2] as f32;
            let G = 0.349 * p[0] as f32 + 0.686 *p[1] as f32 + 0.168 * p[2] as f32;
            let B = 0.272 * p[0] as f32 + 0.534 *p[1] as f32 + 0.131 * p[2] as f32;
            let mut new_p = [clamp(R as i32),clamp( G as i32),clamp( B as i32)];
            
            img.put_pixel(j, i, image::Rgb(new_p));
        }
    }
    img.save("sepia.png"); 
}