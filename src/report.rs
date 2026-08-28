use crate::model::{Comparison, ProofBundle, RunStatus};
use serde::Serialize;

const CSS: &str = r#"
:root{color-scheme:light dark;--bg:#efede4;--surface:#d8d3c5;--raised:#f8f6ef;--text:#171a16;--muted:#555c51;--moss:#365b32;--moss-text:#fff;--lichen:#bbcf72;--rust:#9b3528;--line:#7a7f74}*{box-sizing:border-box}html{font-family:Arial,"Helvetica Neue",sans-serif;background:var(--bg);color:var(--text)}body{margin:0;font-size:16px;line-height:1.55}main,header,footer{width:min(1100px,calc(100% - 32px));margin-inline:auto}header{padding:48px 0 24px;border-bottom:2px solid var(--text)}h1{font-size:clamp(2rem,7vw,4.8rem);line-height:.92;letter-spacing:-.045em;margin:8px 0 24px;max-width:12ch}h2{font-size:1.35rem;margin:0}p{max-width:68ch}.eyebrow,.meta,code,pre,.step-id,.stamp{font-family:ui-monospace,SFMono-Regular,Consolas,monospace;font-variant-numeric:tabular-nums}.eyebrow{font-size:.78rem;text-transform:uppercase;letter-spacing:.14em}.summary{display:flex;flex-wrap:wrap;gap:16px;margin:24px 0 0;padding:0;list-style:none}.summary li{min-width:112px;border-left:4px solid var(--moss);padding:4px 12px}.summary strong{display:block;font:700 1.6rem ui-monospace,SFMono-Regular,Consolas,monospace}.workflow{position:relative;padding:40px 0 24px}.workflow:before{content:"";position:absolute;left:27px;top:32px;bottom:20px;width:4px;background:var(--moss)}.step{position:relative;margin:0 0 24px 64px;background:var(--raised);border:2px solid var(--text);box-shadow:6px 6px 0 var(--surface);padding:20px}.step:before{content:attr(data-number);position:absolute;left:-62px;top:14px;width:44px;height:44px;display:grid;place-items:center;background:var(--lichen);color:#171a16;border:2px solid var(--text);font:700 1rem ui-monospace,SFMono-Regular,Consolas,monospace}.step-head{display:flex;gap:12px;align-items:center;justify-content:space-between}.stamp{padding:4px 10px;border:2px solid currentColor;font-weight:700;text-transform:uppercase}.pass{color:var(--moss)}.fail,.changed{color:var(--rust)}.grid{display:grid;grid-template-columns:1fr 1fr;gap:16px;margin-top:16px}.well{background:var(--surface);padding:12px;border-left:3px solid var(--line);min-width:0}.well h3{font-size:.8rem;text-transform:uppercase;letter-spacing:.08em;margin:0 0 8px}.route{overflow-wrap:anywhere}details{margin-top:12px}summary{cursor:pointer;min-height:44px;display:flex;align-items:center;font-weight:700}pre{font-size:.78rem;line-height:1.55;white-space:pre-wrap;overflow-wrap:anywhere;margin:0;background:var(--text);color:var(--raised);padding:12px}.assertion{display:grid;grid-template-columns:24px 1fr;gap:8px;border-top:1px solid var(--line);padding:10px 0}.assertion:first-child{border-top:0}.assertion b{font-family:ui-monospace,SFMono-Regular,Consolas,monospace}.empty{margin:40px 0;padding:32px;border:2px dashed var(--line)}footer{padding:32px 0 56px;color:var(--muted)}a{color:var(--moss);text-underline-offset:3px}:focus-visible{outline:3px solid var(--lichen);outline-offset:3px}.skip{position:absolute;left:-9999px;top:8px;background:var(--text);color:var(--raised);padding:12px;z-index:2}.skip:focus{left:8px}.change{margin:0 0 24px;padding:20px;border:2px solid var(--text);background:var(--raised);box-shadow:6px 6px 0 var(--surface)}.change-grid{display:grid;grid-template-columns:1fr 1fr;gap:16px}.label{font-size:.75rem;text-transform:uppercase;letter-spacing:.12em;font-weight:700}@media(max-width:640px){header{padding-top:28px}.grid,.change-grid{grid-template-columns:1fr}.workflow:before{left:21px}.step{margin-left:52px;padding:16px}.step:before{left:-50px;width:38px;height:38px}.step-head{align-items:flex-start;flex-direction:column}.summary{display:grid;grid-template-columns:1fr 1fr;width:100%}}@media(prefers-color-scheme:dark){:root{--bg:#171a17;--surface:#252a25;--raised:#303630;--text:#f1efe6;--muted:#b8c0b3;--moss:#a9c978;--moss-text:#11150f;--lichen:#c9dd82;--rust:#ff9a86;--line:#71796f}}@media(prefers-reduced-motion:reduce){*,*:before,*:after{scroll-behavior:auto!important;transition:none!important;animation:none!important}}
"#;

pub(crate) fn render_proof(proof: &ProofBundle) -> String {
    let status = match proof.result {
        RunStatus::Passed => "Passed",
        RunStatus::Failed => "Failed",
    };
    let status_class = match proof.result {
        RunStatus::Passed => "pass",
        RunStatus::Failed => "fail",
    };
    let steps = proof.steps.iter().enumerate().map(|(index, step)| {
        let assertions = if step.assertions.is_empty() {
            "<p>No success criteria declared.</p>".to_owned()
        } else {
            step.assertions.iter().map(|assertion| {
                let mark = if assertion.passed { "✓" } else { "×" };
                let class = if assertion.passed { "pass" } else { "fail" };
                format!("<div class=\"assertion\"><b class=\"{class}\">{mark}</b><div><code>{}</code><br><span class=\"meta\">actual {} · expected {}</span></div></div>", escape(&assertion.condition), value(&assertion.actual), value(&assertion.expected))
            }).collect()
        };
        let step_status = match step.result { RunStatus::Passed => "Pass", RunStatus::Failed => "Fail" };
        let step_class = match step.result { RunStatus::Passed => "pass", RunStatus::Failed => "fail" };
        format!(r#"<article class="step" data-number="{number}"><div class="step-head"><div><span class="eyebrow">Step {number}</span><h2>{id}</h2></div><span class="stamp {step_class}">{step_status}</span></div><div class="grid"><section class="well"><h3>Request</h3><div class="route"><b>{method}</b> <code>{url}</code></div><details><summary>Captured request</summary><pre>{request}</pre></details></section><section class="well"><h3>Response</h3><div><b>HTTP {status}</b></div><details><summary>Captured response</summary><pre>{response}</pre></details></section></div><details open><summary>Assertions ({assertion_count})</summary>{assertions}</details><details><summary>Outputs ({output_count})</summary><pre>{outputs}</pre></details></article>"#,
            number = index + 1,
            id = escape(&step.step_id),
            method = escape(&step.request.method),
            url = escape(&step.request.url),
            request = pretty(&step.request),
            status = step.response.status,
            response = pretty(&step.response),
            assertion_count = step.assertions.len(),
            output_count = step.outputs.len(),
            outputs = pretty(&step.outputs),
        )
    }).collect::<String>();
    document(
        &format!("{} · Arazzo proof", proof.workflow_id),
        &format!(
            r#"<header><span class="eyebrow">Arazzo proof / {environment}</span><h1>{workflow}</h1><p>Evidence captured from <code>{source}</code>. Sensitive values were removed before this report was written.</p><ul class="summary"><li><span>Result</span><strong class="{status_class}">{status}</strong></li><li><span>Steps</span><strong>{step_count}</strong></li><li><span>Passed</span><strong>{passed}</strong></li><li><span>Failed</span><strong>{failed}</strong></li></ul></header><main id="main"><section class="workflow" aria-label="Workflow evidence">{steps}</section></main><footer>Generated locally by Arazzo Proof Runner · proof schema <code>{schema}</code></footer>"#,
            environment = escape(&proof.environment),
            workflow = escape(&proof.workflow_id),
            source = escape(&proof.source_file),
            step_count = proof.summary.steps,
            passed = proof.summary.passed,
            failed = proof.summary.failed,
            schema = escape(&proof.schema_version)
        ),
    )
}

pub(crate) fn render_comparison(comparison: &Comparison) -> String {
    let status = if comparison.changed {
        "Changed"
    } else {
        "Unchanged"
    };
    let class = if comparison.changed {
        "changed"
    } else {
        "pass"
    };
    let changes = if comparison.changes.is_empty() {
        "<section class=\"empty\"><h2>No changes</h2><p>The captured requests, responses, outputs, and assertions match the baseline.</p></section>".to_owned()
    } else {
        comparison.changes.iter().map(|change| format!(r#"<article class="change"><span class="eyebrow">{step}</span><h2>{field}</h2><div class="change-grid"><section><p class="label">Baseline</p><pre>{before}</pre></section><section><p class="label">Current</p><pre>{after}</pre></section></div></article>"#,
            step = escape(&change.step_id), field = escape(&change.field), before = optional_value(change.before.as_ref()), after = optional_value(change.after.as_ref()))).collect()
    };
    document(
        "Arazzo proof comparison",
        &format!(
            r#"<header><span class="eyebrow">Proof comparison</span><h1>{baseline} → {current}</h1><p>Differences between two redacted <code>proof.json</code> files.</p><ul class="summary"><li><span>Result</span><strong class="{class}">{status}</strong></li><li><span>Added</span><strong>{added}</strong></li><li><span>Removed</span><strong>{removed}</strong></li><li><span>Modified</span><strong>{modified}</strong></li></ul></header><main id="main"><section class="workflow" aria-label="Evidence changes">{changes}</section></main><footer>Generated locally by Arazzo Proof Runner · comparison schema <code>{schema}</code></footer>"#,
            baseline = escape(&comparison.baseline_workflow),
            current = escape(&comparison.current_workflow),
            added = comparison.summary.added,
            removed = comparison.summary.removed,
            modified = comparison.summary.modified,
            schema = escape(&comparison.schema_version)
        ),
    )
}

fn document(title: &str, content: &str) -> String {
    format!(
        r##"<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="color-scheme" content="light dark"><meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'; img-src data:"><title>{}</title><style>{CSS}</style></head><body><a class="skip" href="#main">Skip to evidence</a>{content}</body></html>"##,
        escape(title)
    )
}

fn pretty(value: &impl Serialize) -> String {
    escape(&serde_json::to_string_pretty(value).unwrap_or_else(|_| "null".to_owned()))
}

fn value(value: &serde_json::Value) -> String {
    escape(&serde_json::to_string(value).unwrap_or_else(|_| "null".to_owned()))
}

fn optional_value(value: Option<&serde_json::Value>) -> String {
    value.map(pretty).unwrap_or_else(|| "—".to_owned())
}

fn escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
