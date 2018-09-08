pub struct Screen {
    title: String,
    clear: String
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

        let clear = r#"
        #######################
        #                     #
        #  g a m e c l e a r  #
        #                     #
        #######################
        "#.to_string();

        Screen{ title, clear }
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

    pub fn show_clear(&self) {
        self.newln();
        self.newln();
        self.newln();
        self.newln();
        self.showln(&self.clear);
    }

}