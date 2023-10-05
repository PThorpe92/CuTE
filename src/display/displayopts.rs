/// Here are the options that require us to display a box letting
/// the user know that they have selected that option.
#[derive(Debug, Clone, PartialEq)]
pub enum DisplayOpts {
    Verbose,
    // TODO: support more headers
    Headers((String, String)),
    URL(String),
    Outfile(String),
    SaveCommand,
    Response(String),
    RecDownload(usize),
    Auth(String),
}
