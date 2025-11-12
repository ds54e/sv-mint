use crate::sv::cst_ir::CstIr;
use crate::sv::model::{AstSummary, ParseArtifacts};
use crate::types::Stage;
use serde::Serialize;

pub enum StagePayload<'a> {
    RawText(&'a str),
    PpText {
        text: &'a str,
        defines: &'a [crate::sv::model::DefineInfo],
    },
    Cst {
        cst_ir: Option<&'a CstIr>,
        has_cst: bool,
    },
    Ast(&'a AstSummary),
}

pub fn payload_for<'a>(stage: &Stage, artifacts: &'a ParseArtifacts) -> StagePayload<'a> {
    match stage {
        Stage::RawText => StagePayload::RawText(&artifacts.raw_text),
        Stage::PpText => StagePayload::PpText {
            text: &artifacts.pp_text,
            defines: &artifacts.defines,
        },
        Stage::Cst => StagePayload::Cst {
            cst_ir: artifacts.cst_ir.as_ref(),
            has_cst: artifacts.has_cst,
        },
        Stage::Ast => StagePayload::Ast(&artifacts.ast),
    }
}

impl<'a> Serialize for StagePayload<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            StagePayload::RawText(text) => RawTextPayload { text }.serialize(serializer),
            StagePayload::PpText { text, defines } => PpTextPayload { text, defines }.serialize(serializer),
            StagePayload::Cst { cst_ir, has_cst } => {
                if let Some(ir) = cst_ir {
                    CstInlinePayload {
                        mode: "inline",
                        cst_ir: ir,
                    }
                    .serialize(serializer)
                } else {
                    CstNonePayload {
                        mode: "none",
                        has_cst: *has_cst,
                    }
                    .serialize(serializer)
                }
            }
            StagePayload::Ast(ast) => ast.serialize(serializer),
        }
    }
}

#[derive(Serialize)]
struct RawTextPayload<'a> {
    text: &'a str,
}

#[derive(Serialize)]
struct PpTextPayload<'a> {
    text: &'a str,
    defines: &'a [crate::sv::model::DefineInfo],
}

#[derive(Serialize)]
struct CstInlinePayload<'a> {
    #[serde(rename = "mode")]
    mode: &'static str,
    #[serde(rename = "cst_ir")]
    cst_ir: &'a CstIr,
}

#[derive(Serialize)]
struct CstNonePayload {
    #[serde(rename = "mode")]
    mode: &'static str,
    has_cst: bool,
}
