#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_mut)]
use std::error::Error;
use utubeget::file::read_urls;
use utubeget::youtube::get_book_name;

// test
use utubeget::filter::filter;

// TODO
// Read file with urls
// loop over urls
// Get name of video from url
// convert url into latin and create folder with such name with no spaces etc
// load only audio type quality and convert it to mp3
// if error, log to file this url


fn main() -> Result<(), Box<dyn Error>> {
    //let book_urls = read_urls().unwrap();
    //dbg!(&book_urls);

//    let output = Command::new("/usr/bin/youtube-dl")
//        .arg("-e")
//        .arg("https://www.youtube.com/watch?v=kGOMFlkv2pE")
//        .output()
//        .unwrap();
//    let txt = String::from_utf8(output.stdout).unwrap();
//    println!("{}", &txt);

    let test_link: String = String::from("https://www.youtube.com/watch?v=kGOMFlkv2pE");
    let test_name: String = String::from("❗ЛЮДИ КРИЧАТЬ ПІД ЗАВАЛАМИ! Шокуючі кадри з місця прильоту російської ракети у Дніпрі#дніпро #shorts");

    //let test = get_book_name(test_link);
    //println!("{}", test);

    filter( test_name );


    //urls.iter().try_for_each(

    Ok(())
}
