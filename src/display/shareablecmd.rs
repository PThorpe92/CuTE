#[derive(Debug, Clone, PartialEq)]
pub struct ShareableCommand {
    command: String,
    url: String,
    headers: Vec<(String, String)>,
    outfile: String,
    verbose: bool,
}

impl ShareableCommand {
    pub fn new() -> Self {
        Self {
            command: "".to_string(),
            url: "".to_string(),
            headers: Vec::new(),
            outfile: "".to_string(),
            verbose: false,
        }
    }

    pub fn set_command(&mut self, command: String) {
        self.command = command;
    }

    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_headers(&mut self, headers: Vec<(String, String)>) {
        self.headers = headers;
    }

    pub fn push_header(&mut self, header: (String, String)) {
        self.headers.push(header);
    }

    pub fn set_outfile(&mut self, outfile: String) {
        self.outfile = outfile;
    }

    pub fn render_command_str(&self) -> Option<String> {
        if self.command.is_empty() {
            return None;
        }

        if self.url.is_empty() {
            return None;
        }

        // This assembles the simplest possible command string
        let mut command_str = self.command.clone();

        // Check For Verbose Flag
        if self.verbose {
            // Verbose Flag Including Whitespace
            command_str.push_str(" -v");
        }

        // Whitespace
        command_str.push(' ');
        // URL
        command_str.push_str(&self.url);

        // Next We Check For Headers
        if !self.headers.is_empty() {
            for (key, value) in &self.headers {
                // Whitespace
                command_str.push_str("");
                // Header Flag
                command_str.push_str("-H ");
                // Open Quote
                command_str.push('\"');
                // Header Key
                command_str.push_str(key);
                // Delimiter
                command_str.push(':');
                // Header Value
                command_str.push_str(value);
                // Close Quote
                command_str.push('\"');
            }
        }

        // Check For Outfile
        if !self.outfile.is_empty() {
            // Whitespace
            command_str.push(' ');
            // Outfile Flag
            command_str.push_str(" -o ");
            // Outfile Name
            command_str.push_str(&self.outfile);
        }

        // Return Command String
        Some(command_str)
    }
}
