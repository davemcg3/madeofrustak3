#[derive(strum_macros::ToString, Debug)] // for output
pub enum Following {
    IsNot,
    Is,
}

// Following isn't modeled correctly here. There should be a relationship entity between 2 tweeters
// that explains who is following whom
pub struct Tweeter<'a> {
    handle: String,
    location: String,
    website: String,
    following: &'a Following,
    display_vowels: bool,
}

// No idea how these lifetimes work but it made the program compile. Needed them when I stored
// following as a reference instead of a value
impl<'a> Tweeter<'a>{
    fn new(handle: String, location: String, website: String, following: &'a Following, display_vowels: bool) -> Tweeter<'a> {
        Tweeter {
            handle: handle.to_string(),
            location: location.to_string(),
            website: website.to_string(),
            following: &following,
            display_vowels,
        }
    }

    fn no_vowels(&self, replace: char) -> String {
        let mut output = String::from("");
        for (_i, c) in self.handle.chars().enumerate() { // unused i
            output.push(match c {
                'a' => replace,
                'e' => replace,
                'i' => replace,
                'o' => replace,
                'u' => replace,
                _ => c,
            });
        }
        output
    }

    pub fn get_handle(&self) -> String {
        self.handle.to_string()
    }

    pub fn to_string(&self, other_user_handle:String) -> String {
        let mut output_handle: String = String::from("");
        let mut output: String = String::from("");
        output_handle.push_str(self.display_handle().as_str());
        output.push_str(&output_handle);
        output.push_str(", ");
        output.push_str(&self.location);
        output.push_str(", ");
        output.push_str(&self.website);
        output.push_str(", ");
        output.push_str(&format!("{} {} following {}", output_handle, &self.following.to_string(), other_user_handle));

        output
    }

    pub fn display_handle(&self) -> String {
        let mut output_handle: String = String::from("");
        if !self.display_vowels {
            output_handle.push_str(&self.no_vowels('*')); // this char should be configurable per user
        } else {
            output_handle.push_str(&self.handle);
        }
        output_handle.to_string()
    }
}

pub fn create_tweeter (handle:String, location:String, website:String, following:&'static Following, display_vowels:bool) -> Tweeter<'static> {
    Tweeter::new(handle, location, website, following, display_vowels)
}