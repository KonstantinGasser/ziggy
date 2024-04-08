use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub(crate) struct Index {
    pub(crate) rooms: Vec<String>,
}

#[derive(Template)]
#[template(path = "hangout_list.html")]
pub(crate) struct HangoutList {
    pub(crate) rooms: Vec<String>,
}

#[derive(Template)]
#[template(path = "chat.html")]
pub(crate) struct Chat {
    pub(crate) hangout_id: String, // will crash if hangout name has space - would need url
    // encoding
    pub(crate) user_handle: String,
}
