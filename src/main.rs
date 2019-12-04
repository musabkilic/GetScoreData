extern crate reqwest;
extern crate scraper;

use std::io::Read;
use scraper::Html;
use scraper::Selector;

const TEAM : &str = "GALATASARAY";
const GET_SCORE_URL : &str = "http://www.sportoto.gov.tr/Match/Results?WeekId=";
const GET_WEEKS_URL : &str = "http://www.sportoto.gov.tr/Match/ListOfWeekMatches";


fn get_score(week: u32) -> (){
    let mut res = reqwest::get(&format!("{}{}", GET_SCORE_URL, week)).unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).expect("[ERROR] Couldn't get data from the website!");

    let fragment = Html::parse_fragment(&body);
    let selector = Selector::parse(r#"tr[class="info"]"#).unwrap();

    for element in fragment.select(&selector) {
        let text = element.text().collect::<Vec<_>>();
        let teams = text[3].trim();
        let scores = text[5].trim();
        if teams.contains(TEAM) && scores.len() > 0{
            println!("{} : {}", teams, scores);
        }
    }
}

fn get_week_data() -> Vec<u32>{
    let mut weeks : Vec<u32> = Vec::new();

    let mut res = reqwest::get(GET_WEEKS_URL).unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).expect("[ERROR] Couldn't get data from the website!");

    let fragment = Html::parse_fragment(&body);
    let selector0 = Selector::parse("tr").unwrap();
    let selector1 = Selector::parse("a").unwrap();

    for element0 in fragment.select(&selector0) {
        for element1 in element0.select(&selector1){
            let href = element1.value().attr("href");
            match href{
                None => (),
                Some(x) => {
                    if x.contains("/Match/Results?WeekId="){
                        weeks.push(x[22..25].parse().unwrap());
                    }
                },
            }
        }
    }
    return weeks;
}

fn main(){
    println!("LATEST SCORES!!!\n");

    let weeks = get_week_data();
    for week in weeks.into_iter(){
        get_score(week);
    }
}
