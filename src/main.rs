use std::io::{self, Write};
use std::string::String;
use raster::{self, Color};
//use raster::editor::blend;
use image::{self, GenericImageView, GenericImage, Rgba,};
use std::fs;
use rand::Rng;
use imageproc;

#[derive(Debug)]
struct Object{
    spr:u32,
    x:i32,
    y:i32,
    rot:i32,
    color:Rgba<u8> ,
    size:f32 //1.0 means original size
}

// random rot, pos, color and size)

fn _testloop(img1:raster::Image){
    let ylen = img1.height;
    let xlen = img1.width;
    println!("image dimensions: {},{}",xlen,ylen);
    
    let mut y = 0;
    let mut x = 0;
    loop { //y loop
        if y >= ylen{
            break
            println!("broke the main Y loop");
        }

        x = 0;
        loop{

            if x >= xlen{
                break
            }

            let color = img1.get_pixel(x, y).expect("out of bounds error");
            let r = color.r;
            let g = color.g;
            let b = color.b;
            
            println!("({r},{g},{b}) in {x},{y}");

            x += 1;
        }

        y += 1;
    }
}


fn _compare2images(wantedimg:raster::Image,experimentalimg:raster::Image) -> Result<u32,bool>{
    println!("Comparing images started");
    let ylen = wantedimg.height;
    let xlen = wantedimg.width;
    let mut debugimg = raster::Image::blank(xlen, ylen);
    let mut score:u32 = 0;
    //println!("image dimensions: {},{}",xlen,ylen);
    
    let ylen2 = experimentalimg.height;
    let xlen2 = experimentalimg.width;
    //println!("wantedimg:{},{} || experimentalimg:{},{}",xlen,ylen,xlen2,ylen2);
    
    if xlen != xlen2 || ylen != ylen2 {
        return Err(false);
    }


    let mut y = 0;
    let mut x = 0;
    loop { //y loop
        if y >= ylen{
            break
        }

        x = 0;
        loop{

            if x >= xlen{
                break
            }

            let wantedcolor = wantedimg.get_pixel(x, y).expect("out of bounds error");
            let wr = wantedcolor.r; //wanted r
            let wg = wantedcolor.g; //wanted g
            let wb = wantedcolor.b; //wanted b

            let expcolor = experimentalimg.get_pixel(x, y).expect("out of bounds error");
            let er = expcolor.r; //experimental r
            let eg = expcolor.g; //experimental g
            let eb = expcolor.b; //experimental b

            // r1 = wr


            let rrr   = wr as i32 - er as i32;
            let ggg = wg as i32 - eg as i32;
            let bbb  = wb as i32 - eb as i32;

            let rdiff = rrr.abs();
            let gdiff = ggg.abs();
            let bbb = bbb.abs();
            
            let totaldiff = rdiff+gdiff+bbb; // i think total diff is working
            let added_score = (255*3)-totaldiff;
            score += added_score as u32;

            // How does scoring work?
            // first calculate the difference and then get its absolute value
            // then subtract 255*3 by difference
            // so the exact color is 765 score and the exact opposite color is 0 score

            debugimg.set_pixel(x, y, Color::rgb(totaldiff as u8,(added_score/3) as u8,0)).expect("debug writing pixel fail");
            // in debug image red means exact opposite and green means the same color

            //println!("total score is {}",added_score);
            
            x += 1;
        }

        y += 1;
    }
    raster::save(&debugimg, "debug.png").expect("cannot write debug image!");
    return Result::Ok(score);

}


fn compare2images_image(wantedimg:&image::DynamicImage,experimentalimg:&image::DynamicImage) -> Result<u32,bool>{
    let st = std::time::Instant::now();
    //println!("Comparing images started");
    let ylen = wantedimg.height(); //wantedimg.height;
    let xlen = wantedimg.width();
    let mut debugimg = image::DynamicImage::new_rgb16(xlen, ylen);
    let mut score:u32 = 0;
    
    let ylen2 = experimentalimg.height();
    let xlen2 = experimentalimg.width();
    
    if xlen != xlen2 || ylen != ylen2 {
        println!("Dimensions doesn't match, exiting...");
        return Err(false);
    }


    let mut y:u32 = 0;
    let mut x:u32 = 0;
    loop { //y loop
        if y >= ylen{
            break
        }

        x = 0;
        loop{

            if x >= xlen{
                break
            }

            let wantedcolor = wantedimg.get_pixel(x, y);
            let wr = wantedcolor[0]; //wanted r
            let wg = wantedcolor[1]; //wanted g
            let wb = wantedcolor[2]; //wanted b 
            // wa = wantedcolor[3]; //wanted alpha

            let expcolor = experimentalimg.get_pixel(x, y);
            let er = expcolor[0]; //experimental r
            let eg = expcolor[1]; //experimental g
            let eb = expcolor[2]; //experimental b

            
            let rrr   = wr as i32 - er as i32;
            let ggg = wg as i32 - eg as i32;
            let bbb  = wb as i32 - eb as i32;

            let rdiff = rrr.abs();
            let gdiff = ggg.abs();
            let bbb = bbb.abs();
            
            let totaldiff = rdiff+gdiff+bbb; // i think total diff is working
            let added_score = (255*3)-totaldiff;
            score += added_score as u32;
            
            // How does scoring work?
            // first calculate the difference and then get its absolute value
            // then subtract 255*3 by difference
            // so the exact color is 765 score and the exact opposite color is 0 score

            let color = image::Rgba([totaldiff as u8, (added_score/3) as u8, 0,255]);
            debugimg.put_pixel(x, y, color);
            // in debug image red means exact opposite and green means the same color

            
            x += 1;
        }

        y += 1;
    }
    image::DynamicImage::save(wantedimg, "compare2img_imgdebug_wantedimg.png");
    image::DynamicImage::save(experimentalimg, "compare2img_imgdebug_experimentalimg.png");
    image::DynamicImage::save(&debugimg, "debug.png").expect("cant save debugimg");
    //image::save_buffer("debug.png", debugimg, xlen, ylen, image::ColorType::Rgba16).expect("cant write debug img");
    //println!("It Took {:?}",st.elapsed());
    return Result::Ok(score);

}


fn multiply_img_color(wantedimg:&mut image::DynamicImage,color:Rgba<u8>) -> Result<bool,bool>{
    //println!("multiplying images started");
    let ylen = wantedimg.height(); //wantedimg.height;
    let xlen = wantedimg.width();

    //let mut debuglist:Vec<String> = Vec::new();
    //let mut debugimg = image::DynamicImage::new_rgba16(xlen, ylen);

    let mut y:u32 = 0;
    let mut x:u32 = 0;
    loop { //y loop
        if y >= ylen{
            break
        }

        x = 0;
        loop{

            if x >= xlen{
                break
            }

            let wantedcolor = wantedimg.get_pixel(x, y);
            let wr = wantedcolor[0]; //wanted r
            let wg = wantedcolor[1]; //wanted g
            let wb = wantedcolor[2]; //wanted b 
            let wa = wantedcolor[3]; //wanted alpha

            // these are between 0 and 1
            let waf = wa as f32/255 as f32 ;
            let wrf = (wr as f32/255 as f32)*waf ; 
            let wgf = (wg as f32/255 as f32)*waf ;
            let wbf = (wb as f32/255 as f32)*waf ; 
            
            // since multiply uses an image to add it and we dont have an 2nd image but instead an color, we will use color for img b instead

            let red_result = ((wrf*color[0] as f32)) as u8;
            let green_result = ((wgf*color[1] as f32)) as u8;
            let blue_result = ((wbf*color[2] as f32)) as u8;
            let alpha_result = (waf*255.0) as u8;
            //println!("alphaaaa is {}",waf*255.0);

            let result_pixel = image::Rgba([red_result,green_result,blue_result,alpha_result]);
            // NOTE: KEEP IN MIND ALPHA RESULT YOU MIGHT NEED TO CHANGE HOW ITS CALCULATED

            //debugimg.put_pixel(x, y, result_pixel);
            wantedimg.put_pixel(x, y, result_pixel);

            let add = "(".to_string()+ &red_result.to_string()+ &",".to_string()+ 
                &green_result.to_string()+ &",".to_string()+ &blue_result.to_string()+&",".to_string()+ &alpha_result.to_string()+&")".to_string(); 
            //debuglist.push(add);

            
            x += 1;
        }

        y += 1;
    }
    

    //image::DynamicImage::save(&debugimg, "multiplydebug.png").expect("cannot write multiply debugimg");
    
    //let mut file = fs::File::create("multiplydebug.txt").unwrap();
    //for i in &debuglist{
    //    write!(file, "{}",i).expect("cant write to file");
    //}
    

    
    return Result::Ok(true);

}

fn make_run(wantedimg:&image::DynamicImage,experimentalimage:&mut image::DynamicImage,currentrun:&mut i32,
        objects:&mut Vec<(Object,i32)>,sprlist:&Vec<image::DynamicImage>,wantedobjamount:i32) {

    let mut curobjnum = 0; //currentobj number
    loop {
    let mut copied_experimental = experimentalimage.clone();
    // basically, we will create an object that has position and an sprite id, 
    // then multiply(change its color) of the experimentalimg, then calculate score
    // then add both the object and the score to the objlist
    let mut rng = rand::thread_rng();

    let xlen = wantedimg.width() as i32;
    let ylen = wantedimg.height() as i32;
    let randomsprval = rng.gen_range(0..=sprlist.len()-1 );
    let randomx = rng.gen_range(0..xlen);
    let randomy = rng.gen_range(0..ylen);
    let randomrot = rng.gen_range(0..360);
    let size = rng.gen_range(0.1..5.0);
    let mut objscore = 0;
    let color = image::Rgba([rng.gen_range(0..255),rng.gen_range(0..255),
     rng.gen_range(0..255),255]);


    let mut experimentspr = sprlist[randomsprval].clone();
    let expwidth = experimentspr.width();
    let expheight = experimentspr.height();
    //image::DynamicImage::save(&experimentspr, "experimentspr.png").expect("cannot write experiment image");

    // multiply(apply color change to sprite)
    let _result = multiply_img_color(&mut experimentspr, color).unwrap_or(false);
    
    let outputbuffer = imageproc::geometric_transformations::rotate(&experimentspr.into_rgba16(),
     (expwidth as f32/ 1.7,expheight as f32/ 1.7 ),randomrot as f32, 
    imageproc::geometric_transformations::Interpolation::Bicubic,image::Rgba([255,182,193,255]));

    //image::DynamicImage::ImageRgba16(outputimg);

    //outputimg = image::DynamicImage::resize_exact(&outputimg, nwidth, nheight, filter)

    let mut outputimg = image::DynamicImage::ImageRgba16(outputbuffer);
    
    //println!("wantedimg size is{:?}, outputimg size is{:?}",wantedimg.dimensions(),outputimg.dimensions());
    
    outputimg = image::DynamicImage::resize_exact(&outputimg, (expwidth as f32*size) as u32, (expheight as f32*size) as u32, image::imageops::FilterType::Triangle);

    //let path = "rotation test".to_string() + &currentrun.to_string() + ".png"; 
    //outputimg.save(path).expect("Rotation test image save failed");
    

    let blitx = rng.gen_range(0..copied_experimental.width());
    let blity = rng.gen_range(0..copied_experimental.height());
    image::imageops::overlay(&mut copied_experimental, &outputimg, blitx as i64, blity as i64);

    objscore = compare2images_image(wantedimg, &copied_experimental).unwrap_or(0);
    
    //copied_experimental.save("copiedexperimentaltest".to_string()+&currentrun.to_string()+".png").expect("biktim");

    if curobjnum % 50 == 0{
        println!("Currently at {} out of {}",curobjnum,wantedobjamount);
    }

    let f = ( Object{spr:randomsprval as u32,x:randomx,y:randomy,rot:randomrot,color,size },objscore as i32);
    //println!("OBJECT IS {:?}",f);
    objects.push(f);
    curobjnum += 1;
    
    if curobjnum >= wantedobjamount{
        break       }
    }
    
    // repeat this 1000 times
    // write the values and object classifications to a debug file
    let mut index = 0;
    let mut highestvalue = -1;
    let mut highestobjindex = 0;
    for thing in objects.iter(){
        if thing.1 > highestvalue{
            // bigger value
            highestvalue = thing.1.clone();
            highestobjindex = index.clone();
        }
        index += 1
    }

    // THIS PART IS FOR ACTUALLY CHANGING THE ORIGINAL EXPERIMENTALIMG
    
    let mut experimentspr = sprlist[objects[highestobjindex].0.spr as usize ].clone();
    let expwidth = experimentspr.width();
    let expheight = experimentspr.height();

    multiply_img_color(&mut experimentspr, objects[highestobjindex].0.color).unwrap_or(false);

    let outputbuffer = imageproc::geometric_transformations::rotate(&experimentspr.into_rgba16(),
     (expwidth as f32/ 1.7,expheight as f32/ 1.7 ),objects[highestobjindex].0.rot as f32, 
     imageproc::geometric_transformations::Interpolation::Bicubic,image::Rgba([255,182,193,255]));

    let mut outputimg = image::DynamicImage::ImageRgba16(outputbuffer);
    
    println!("RESULT wantedimg size is{:?}, outputimg size is{:?}",wantedimg.dimensions(),outputimg.dimensions());
    
    outputimg = image::DynamicImage::resize_exact(&outputimg, (expwidth as f32*objects[highestobjindex].0.size) as u32, 
     (expheight as f32*objects[highestobjindex].0.size) as u32, image::imageops::FilterType::Triangle);

    
    let mut rng = rand::thread_rng();

    let blitx = rng.gen_range(0..expwidth);
    let blity = rng.gen_range(0..expheight);
    image::imageops::overlay(experimentalimage, &outputimg, blitx as i64, blity as i64);
 
    compare2images_image(wantedimg, &experimentalimage).unwrap_or(0);
     
    experimentalimage.save("result/run".to_string()+&currentrun.to_string()+&".png").expect("Cant save output image when saving to result/");

    *currentrun += 1;
    println!("ENDED RUN: The biggest found value: {} and the index is {}",highestvalue,highestobjindex);
    objects.drain(..);

    // add saving images and also try to fix the images going out of screen and stuff like that
    // ALSO IMPORTANT: objects arent being reset after each run, fix that
}


fn main() {
    

    println!("Please enter the path to the image that you want the ai to recreate:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input) //store the string in guess
        .expect("Failed to read."); //output message in case it fails to read it

    let path = input.as_str().trim();

    let mut objl:Vec<(Object,i32)> = Vec::new();
    let wantimg = image::open(path).expect("Can't open image");

    let paths = fs::read_dir("./sprites/").unwrap();
    let mut sprlist:Vec<image::DynamicImage> = Vec::new();

    let mut sprcount = 0;
    for path in paths {
        let pathstring  = path.unwrap().path().display().to_string();
        let rawimg = image::open(pathstring).unwrap();
        let raw_w = rawimg.width();
        let raw_h = rawimg.height();


        let mut width = 1;
        let mut height = 1;
        let mut excess:f32 = 0.0;
        if raw_w >= raw_h{
            excess = raw_w as f32 * 0.25;
            width = (raw_w as f32 * 1.5) as i32;
            height = (raw_w as f32 * 1.5) as i32;
        } else{
            excess = raw_h as f32 * 0.25;
            width = (raw_h as f32 * 1.5) as i32;
            height = (raw_h as f32 * 1.5) as i32;
        }

        let ibuff: image::RgbaImage = image::RgbaImage::new(width as u32, height as u32); //make an buffer filled with 0,0,0,0
        //image::save_buffer("buffersavetest.png", &ibuff, ibuff.width(), ibuff.height(), image::ColorType::Rgba8); 

        let mut new_img = image::DynamicImage::ImageRgba8(ibuff);

        image::imageops::overlay(&mut new_img, &rawimg, excess as i64, excess as i64);
        
        //new_img.save("sprtestsave".to_string()+&sprcount.to_string()+".png").expect("sprtestave couldnt write image");

        sprlist.push(new_img); 

        sprcount += 1;
        
    }
    println!("Successuly loaded images");

    let mut inpnum = String::new();
    println!("Please enter the wanted amount of runs: ");
    io::stdin()
        .read_line(&mut inpnum) //store the string in guess
        .expect("Failed to read."); //outpput message in case it fails to read it

    let wantedrunamount = inpnum.as_str().trim().parse().unwrap_or(10);
    let mut currentrun:i32 = 0;

    inpnum = String::new();
    println!("Please enter the wanted amount of objects per run: ");
    io::stdin()
        .read_line(&mut inpnum) //store the string in guess
        .expect("Failed to read."); //outpput message in case it fails to read it

    let wantedobjamount = inpnum.as_str().trim().parse().unwrap_or(10);

    println!("Starting Run {} ,wantedrun amount:{}, wantedobj per run {}",currentrun,wantedrunamount,wantedobjamount);


    // expimg will be the base experimentalimage, in each run this img will be given, then the make_run will take these 
    // and copy the image for every single object tried, then go on to experiment on it 
    let mut expimg = image::DynamicImage::new_rgba16(wantimg.width(),wantimg.height()); //experimented image

    loop {
        println!("Currentrun = {}, wantedrun = {} ",currentrun,wantedrunamount);
        
        make_run(&wantimg,&mut expimg,&mut currentrun,&mut objl,&sprlist,wantedobjamount);
        
        if currentrun >= wantedrunamount{
            println!("Reached the wanted run amount.");
            break
        }
        
    }

}
