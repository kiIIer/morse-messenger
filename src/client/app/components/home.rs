pub struct HomeComponent {
    heading: String,
    tutorial: String
}

impl Default for HomeComponent {
    fn default() -> Self {
        let tutorial = String::from("Well this is supposed to be a quick tutorial over how the app works.");
        HomeComponent{
            heading: String::from("Morse Code Messenger"),
            tutorial
        }
    }
}