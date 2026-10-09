
use image::ImageBuffer;
fn quantize(colors: [u8;3]) -> [i32;3]{
    return [((colors[0] as i32)/85 *85),((colors[1] as i32)/85 *85) ,((colors[2] as i32)/85 *85)];
     //(color / 51) *51 
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
fn add_errors(errors:[i32;3], p: [u8;3], coef: i32)-> [u8;3]{
    let new_r = clamp(errors[0]*coef/16 + p[0] as i32);
    let new_g = clamp(errors[1]*coef/16 + p[1] as i32);
    let new_b = clamp(errors[2]*coef/16 + p[2] as i32);
    return [new_r,new_g, new_b];

}
pub fn floyd(){
    let mut img  = image::open("hrse.jpg").unwrap().to_rgb8();
    for i in 0..img.height(){
        for j in 0..img.width(){
            let p = img.get_pixel(j, i).0;
            let new_p = quantize(p);
            img.put_pixel(j, i, image::Rgb([new_p[0] as u8, new_p[1] as u8, new_p[2] as u8]));
            let errors = [p[0] as i32 -new_p[0], p[1] as i32 -new_p[1], p[2] as i32 -new_p[2]];
            if j+1 < img.width() && i+1 < img.height(){
                img.put_pixel(j+1, i+1, image::Rgb(add_errors(errors, img.get_pixel(j+1, i+1).0, 1)));
            }
            if j  > 0 && i+1 < img.height() {
                img.put_pixel(j-1, i+1, image::Rgb(add_errors(errors, img.get_pixel(j-1, i+1).0, 3 )));
            }
            if i+1 < img.height() {
                img.put_pixel(j, i+1, image::Rgb(add_errors(errors, img.get_pixel(j, i+1).0, 5 )));
            }
            if j+1 < img.width(){
                img.put_pixel(j+1, i, image::Rgb(add_errors(errors, img.get_pixel(j+1, i).0, 7)));
            }
            //println!("{} {} {} / {} {} {}",new_p[0], new_p[1], new_p[2], p[0], p[1],p[2]);
        }
    }
    img.save("output2.png");

}

fn calculate_contrast(x:i32, y:i32, img:&ImageBuffer<image::Luma<u8>,Vec<u8>> ) -> u8{
    let mut vec: Vec<u8> = Vec::new();
    let w: i32 = img.width() as i32;
    let h: i32 = img.height() as i32;

    for i in -1..2{
        for j in -1..2 {
            if x+j > w-1 || x+j < 0 || y+i > h-1 || y+i < 0{
                continue;
            } else {
                vec.push(img.get_pixel((x+j) as u32, (y+i) as u32).0[0]);
            }
        }
    }
    let max = vec.iter().max().unwrap();
    let min = vec.iter().min().unwrap();
    return max-min;
}
pub fn adapt_disering(){
    let mut img: ImageBuffer<image::Luma<u8>, Vec<u8>>  = image::open("hrse.jpg").unwrap().to_luma8();
    let mut out_img: ImageBuffer<image::Luma<u8>, Vec<u8>> = ImageBuffer::new(img.width(), img.height());
    let buyer_matrix: [[u32; 4]; 4] = [[0,8,2,10],[12,4,14,6], [3,11,1,9],[15,7,13,5]];
    for i in 0..img.height(){
        for j in 0..img.width(){
            let p = img.get_pixel(j, i).0[0];
            let contrast = calculate_contrast(j as i32, i as i32, &img);
            let strength  = 1.0 - contrast as f32/255.0;
            let b_threshold = ((buyer_matrix[(i%4) as usize][(j%4) as usize]*256)/16) as i32;
            let threshold = 128.0 + strength*(b_threshold -128) as f32;
           // println!("{}", threshold);
            if p as f32 > threshold {
                out_img.put_pixel(j, i, image::Luma([255]));
               } else {
                out_img.put_pixel(j, i, image::Luma([0]));
            }
        }
    }
    out_img.save("output3.png");
}
pub fn common_disering(){
    let buyer_matrix: [[u32; 4]; 4] = [[0,8,2,10],[12,4,14,6], [3,11,1,9],[15,7,13,5]];
    let mut img  = image::open("hrse.jpg").unwrap().to_luma8();

    for i in 0..img.height(){
        for j in 0..img.width(){
           let p = img.get_pixel(j, i).0[0]; 
           let threshold = ((buyer_matrix[(i%4) as usize][(j%4) as usize]*256)/16) as u8;
          
           if p > threshold {
            img.put_pixel(j, i, image::Luma([255]));
           } else {
            img.put_pixel(j, i, image::Luma([0]));
           }
        }
    }
   
    img.save("output1.png");
}
