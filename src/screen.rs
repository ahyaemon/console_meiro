pub struct Screen {
    title: String
}

impl Screen {

    pub fn new() -> Screen {
        let title = r#"
        #############################################################
        #                                                           #
        #  Hello, Welcome To The Meiro Of The Super Meironic Meiro  #
        #                                                           #
        #############################################################
        "#.to_string();
        Screen{ title }
    }

    pub fn showln(&self, s: &str) {
        println!("{}", s);
    }

    pub fn newln(&self) {
        println!("");
    }

    // pub fn show(&self, s: &str) {
    //     print!("{}", s);
    // }

    pub fn show_title(&self) {
        self.newln();
        self.newln();
        self.newln();
        self.newln();
        self.showln(&self.title);
    }

}