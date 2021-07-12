use crate::tweeter::{Following, Tweeter, create_tweeter};

mod caesar;
mod tweeter;

const SOURCE_DATA: [(&str, &str, &str, Following, bool); 2] = [("pdghriplvwdn3", "brxu prp", "kwwsv://pdghriplvwdnh.frp/", Following::IsNot, false), ("gdyhpfj3", "Mrqxphqw, CO", "kwwsv://frglqj.gpx.lr/", Following::Is, true)];

fn main() {
    let mut tweeters: Vec<Tweeter> = vec![];
    let mut handles: Vec<String> = vec![];

    // process source data
    for person in SOURCE_DATA.iter() {
        let (handle, location, website, following, display_vowels) = person;
        let decoded_handle: String = caesar::decode(handle.to_string());
        handles.push(decoded_handle.clone());
        tweeters.push(create_tweeter(decoded_handle.clone(), caesar::decode(location.to_string()), caesar::decode(website.to_string()), following, *display_vowels)); // probably could have setup to pass the tuple here and then destructure in the constructor
    }

    // output results
    for tweeter in tweeters.iter() {
        // There's a structural issue here in how I messed up the data modeling. By keeping a
        // separate vector of handles I need them in plain text to compare against the handle of the
        // displayed user which excludes them from the output (desired behavior) but it displays the
        // handle in plain text instead of with the way the user wants the handle displayed. What I
        // _should_ have done was pass a reference to all other tweeters in (instead of just their
        // handle) so that I can have the details to display their handle the way they want it
        // displayed. Fixable, but not worth the effort for _this_ program. C'est la vie.
        for handle in handles.iter() {
            if *handle != tweeter.get_handle() {
                println!("{}", tweeter.to_string(handle.to_string()));
            }
        }
    }
}
