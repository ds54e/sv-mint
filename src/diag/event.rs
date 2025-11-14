#[derive(Clone, Copy, Debug)]
pub enum Event {
    StageStart,
    StageDone,
    PluginInvoke,
    PluginDone,
    PluginTimeout,
    PluginExitNonzero,
    PluginError,
    PluginStderr,
    ParsePreprocessStart,
    ParsePreprocessDone,
    ParseParseStart,
    ParseParseDone,
    ParseAstCollectDone,
}

impl Event {
    pub fn name(&self) -> &'static str {
        match self {
            Event::StageStart => "stage_start",
            Event::StageDone => "stage_done",
            Event::PluginInvoke => "plugin_invoke",
            Event::PluginDone => "plugin_done",
            Event::PluginTimeout => "plugin_timeout",
            Event::PluginExitNonzero => "plugin_exit_nonzero",
            Event::PluginError => "plugin_error",
            Event::PluginStderr => "plugin_stderr",
            Event::ParsePreprocessStart => "parse_preprocess_start",
            Event::ParsePreprocessDone => "parse_preprocess_done",
            Event::ParseParseStart => "parse_parse_start",
            Event::ParseParseDone => "parse_parse_done",
            Event::ParseAstCollectDone => "parse_ast_collect_done",
        }
    }
}

#[derive(Debug)]
pub struct Ev<'a> {
    pub event: Event,
    pub path: &'a str,
    pub stage: Option<&'a str>,
    pub duration_ms: Option<u128>,
    pub exit_code: Option<i32>,
    pub stderr_snippet: Option<&'a str>,
    pub message: Option<&'a str>,
}

impl<'a> Ev<'a> {
    pub fn new(event: Event, path: &'a str) -> Self {
        Self {
            event,
            path,
            stage: None,
            duration_ms: None,
            exit_code: None,
            stderr_snippet: None,
            message: None,
        }
    }

    pub fn with_stage(mut self, stage: &'a str) -> Self {
        self.stage = Some(stage);
        self
    }

    pub fn with_duration_ms(mut self, ms: u128) -> Self {
        self.duration_ms = Some(ms);
        self
    }

    pub fn with_exit_code(mut self, code: i32) -> Self {
        self.exit_code = Some(code);
        self
    }

    pub fn with_stderr_snippet(mut self, snippet: &'a str) -> Self {
        self.stderr_snippet = Some(snippet);
        self
    }

    pub fn with_message(mut self, message: &'a str) -> Self {
        self.message = Some(message);
        self
    }
}
