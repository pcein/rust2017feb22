
use hyper::Client;
use hyper::status::StatusCode;

use myconfig;

pub struct Score(pub i32);
pub struct Question(pub i32);

fn process(status: StatusCode) -> &'static str {
    match status {
        StatusCode::Ok => "Success!",
        _ => "Error! Maybe you tried posting the same entry more than once!",
    }
}

pub fn submit(Question(qn_num): Question, Score(score): Score) {
 
    let url = format!("http://{}:{}/score/{}/{}/{}", 
                      myconfig::SERVER, myconfig::PORT,
                      myconfig::TEAM_NAME, qn_num, score);

    let client = Client::new();
    
    let res = client.post(&url[..]).send();

    println!("");

    match res {
        Ok(res) => println!("Posted score to the server, response: {:?}", process(res.status)),
        Err(_) => println!("Unable to post the score to the server, \
                            maybe the server is down or you have network problems?"),
    }

    println!("");
}
