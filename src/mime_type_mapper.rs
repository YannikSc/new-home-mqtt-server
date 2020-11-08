/// Matching helper implemented for `Fn(String) -> bool`
/// Used to check if the a file is of the desired mime type
pub trait MimeTypeMatcher {
    /// Checks if the given file is of the desired mime type
    /// The type is registered with this matcher in the [MimeTypeMapper].
    fn match_type(&self, file: &String) -> bool;
}

/// Holds registered mime types and matchers
pub struct MimeTypeMapper {
    matcher: Vec<(String, Box<dyn MimeTypeMatcher>)>,
}

/// Implements the [MimeTypeMatcher] witch a check for file extensions
struct ExtensionMatcher(pub String);

impl<F: Fn(&String) -> bool> MimeTypeMatcher for F {
    fn match_type(&self, file: &String) -> bool {
        (self)(file)
    }
}

impl ExtensionMatcher {
    pub fn new(extension: impl ToString) -> Self {
        Self(extension.to_string())
    }
}

impl MimeTypeMatcher for ExtensionMatcher {
    fn match_type(&self, filename: &String) -> bool {
        filename.ends_with(&self.0)
    }
}

impl MimeTypeMapper {
    /// Adds a matcher with a given mime type
    pub fn add_matcher(&mut self, mime_type: impl ToString, matcher: impl MimeTypeMatcher + 'static) {
        self.matcher.push((mime_type.to_string(), Box::new(matcher)));
    }

    /// Matches the given filename
    /// outputs the mimetype which matches the file name
    pub fn match_file(&self, file: impl ToString) -> String {
        let file = file.to_string();

        for (mime_type, matcher) in &self.matcher {
            if matcher.match_type(&file) {
                return mime_type.clone();
            }
        }

        String::from("application/octet-stream")
    }
}

impl Default for MimeTypeMapper {
    fn default() -> Self {
        let mut mapper = Self { matcher: Default::default() };

        // Mime types from https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
        mapper.add_matcher("text/html", ExtensionMatcher::new(".html"));
        mapper.add_matcher("text/html", ExtensionMatcher::new(".htm"));
        mapper.add_matcher("audio/aac", ExtensionMatcher::new(".aac"));
        mapper.add_matcher("application/x-abiword", ExtensionMatcher::new(".abw"));
        mapper.add_matcher("application/x-freearc", ExtensionMatcher::new(".arc"));
        mapper.add_matcher("video/x-msvideo", ExtensionMatcher::new(".avi"));
        mapper.add_matcher("application/vnd.amazon.ebook", ExtensionMatcher::new(".azw"));
        mapper.add_matcher("application/octet-stream", ExtensionMatcher::new(".bin"));
        mapper.add_matcher("image/bmp", ExtensionMatcher::new(".bmp"));
        mapper.add_matcher("application/x-bzip", ExtensionMatcher::new(".bz"));
        mapper.add_matcher("application/x-bzip2", ExtensionMatcher::new(".bz2"));
        mapper.add_matcher("application/x-csh", ExtensionMatcher::new(".csh"));
        mapper.add_matcher("text/css", ExtensionMatcher::new(".css"));
        mapper.add_matcher("text/csv", ExtensionMatcher::new(".csv"));
        mapper.add_matcher("application/msword", ExtensionMatcher::new(".doc"));
        mapper.add_matcher("application/vnd.openxmlformats-officedocument.wordprocessingml.document", ExtensionMatcher::new(".docx"));
        mapper.add_matcher("application/vnd.ms-fontobject", ExtensionMatcher::new(".eot"));
        mapper.add_matcher("application/epub+zip", ExtensionMatcher::new(".epub"));
        mapper.add_matcher("application/gzip", ExtensionMatcher::new(".gz"));
        mapper.add_matcher("image/gif", ExtensionMatcher::new(".gif"));
        mapper.add_matcher("image/vnd.microsoft.icon", ExtensionMatcher::new(".ico"));
        mapper.add_matcher("text/calendar", ExtensionMatcher::new(".ics"));
        mapper.add_matcher("application/java-archive", ExtensionMatcher::new(".jar"));
        mapper.add_matcher("image/jpeg", ExtensionMatcher::new(".jpeg"));
        mapper.add_matcher("image/jpeg", ExtensionMatcher::new(".jpg"));
        mapper.add_matcher("text/javascript", ExtensionMatcher::new(".js"));
        mapper.add_matcher("application/json", ExtensionMatcher::new(".json"));
        mapper.add_matcher("audio/midi audio/x-midi", ExtensionMatcher::new(".mid"));
        mapper.add_matcher("audio/midi audio/x-midi", ExtensionMatcher::new(".midi"));
        mapper.add_matcher("text/javascript", ExtensionMatcher::new(".mjs"));
        mapper.add_matcher("audio/mpeg", ExtensionMatcher::new(".mp3"));
        mapper.add_matcher("video/mpeg", ExtensionMatcher::new(".mpeg"));
        mapper.add_matcher("application/vnd.apple.installer+xml", ExtensionMatcher::new(".mpkg"));
        mapper.add_matcher("application/vnd.oasis.opendocument.presentation", ExtensionMatcher::new(".odp"));
        mapper.add_matcher("application/vnd.oasis.opendocument.spreadsheet", ExtensionMatcher::new(".ods"));
        mapper.add_matcher("application/vnd.oasis.opendocument.text", ExtensionMatcher::new(".odt"));
        mapper.add_matcher("audio/ogg", ExtensionMatcher::new(".oga"));
        mapper.add_matcher("video/ogg", ExtensionMatcher::new(".ogv"));
        mapper.add_matcher("application/ogg", ExtensionMatcher::new(".ogx"));
        mapper.add_matcher("audio/opus", ExtensionMatcher::new(".opus"));
        mapper.add_matcher("font/otf", ExtensionMatcher::new(".otf"));
        mapper.add_matcher("image/png", ExtensionMatcher::new(".png"));
        mapper.add_matcher("application/pdf", ExtensionMatcher::new(".pdf"));
        mapper.add_matcher("application/x-httpd-php", ExtensionMatcher::new(".php"));
        mapper.add_matcher("application/vnd.ms-powerpoint", ExtensionMatcher::new(".ppt"));
        mapper.add_matcher("application/vnd.openxmlformats-officedocument.presentationml.presentation", ExtensionMatcher::new(".pptx"));
        mapper.add_matcher("application/vnd.rar", ExtensionMatcher::new(".rar"));
        mapper.add_matcher("application/rtf", ExtensionMatcher::new(".rtf"));
        mapper.add_matcher("application/x-sh", ExtensionMatcher::new(".sh"));
        mapper.add_matcher("image/svg+xml", ExtensionMatcher::new(".svg"));
        mapper.add_matcher("application/x-shockwave-flash", ExtensionMatcher::new(".swf"));
        mapper.add_matcher("application/x-tar", ExtensionMatcher::new(".tar"));
        mapper.add_matcher("image/tiff", ExtensionMatcher::new(".tif"));
        mapper.add_matcher("image/tiff", ExtensionMatcher::new(".tiff"));
        mapper.add_matcher("video/mp2t", ExtensionMatcher::new(".ts"));
        mapper.add_matcher("font/ttf", ExtensionMatcher::new(".ttf"));
        mapper.add_matcher("text/plain", ExtensionMatcher::new(".txt"));
        mapper.add_matcher("application/vnd.visio", ExtensionMatcher::new(".vsd"));
        mapper.add_matcher("audio/wav", ExtensionMatcher::new(".wav"));
        mapper.add_matcher("audio/webm", ExtensionMatcher::new(".weba"));
        mapper.add_matcher("video/webm", ExtensionMatcher::new(".webm"));
        mapper.add_matcher("image/webp", ExtensionMatcher::new(".webp"));
        mapper.add_matcher("font/woff", ExtensionMatcher::new(".woff"));
        mapper.add_matcher("font/woff2", ExtensionMatcher::new(".woff2"));
        mapper.add_matcher("application/xhtml+xml", ExtensionMatcher::new(".xhtml"));
        mapper.add_matcher("application/vnd.ms-excel", ExtensionMatcher::new(".xls"));
        mapper.add_matcher("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", ExtensionMatcher::new(".xlsx"));
        mapper.add_matcher("text/xml", ExtensionMatcher::new(".xml"));
        mapper.add_matcher("application/vnd.mozilla.xul+xml", ExtensionMatcher::new(".xul"));
        mapper.add_matcher("application/zip", ExtensionMatcher::new(".zip"));
        mapper.add_matcher("video/3gpp", ExtensionMatcher::new(".3gp"));
        mapper.add_matcher("video/3gpp2", ExtensionMatcher::new(".3g2"));
        mapper.add_matcher("application/x-7z-compressed", ExtensionMatcher::new(".7z"));

        mapper
    }
}
