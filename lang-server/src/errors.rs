use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticOptions, DiagnosticServerCapabilities, DiagnosticSeverity, Range,
    WorkDoneProgressOptions,
};
pub const DIAGNOSTIC_CAPACITIES: Option<DiagnosticServerCapabilities> =
    Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
        identifier: None,
        inter_file_dependencies: true,
        workspace_diagnostics: false,
        work_done_progress_options: WorkDoneProgressOptions {
            work_done_progress: None,
        },
    }));

impl crate::Document {
    pub fn errors<'a>(&'a self) -> Vec<Diagnostic> {
        let converter = self.index_converter();
        self.ir()
            .errors()
            .map(|error| Diagnostic {
                range: Range {
                    start: converter(error.span().start),
                    end: converter(error.span().end),
                },
                severity: Some(DiagnosticSeverity::ERROR),
                code: None,
                code_description: None,
                source: Some("compiler".to_string()),
                message: format!("{error}"),
                related_information: None,
                tags: None,
                data: None,
            })
            .collect()
    }
}
