use anyhow::Result;
use clap::{Parser, Subcommand};
use glob::glob;
use regex::Regex;
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{error, info, warn};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate,
    Lint {
        #[arg(short, long)]
        target_dir: String,
        #[arg(short, long)]
        repo_name: String,
        #[arg(short, long)]
        spec_dir: String,
    },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt().json().with_target(false).init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate => generate_docs()?,
        Commands::Lint { target_dir, repo_name, spec_dir } => run_linter(target_dir, repo_name, spec_dir)?,
    }
    Ok(())
}

fn generate_docs() -> Result<()> {
    info!("🚀 (Rust Engine) Generating documentation and AI context...");
    let spec_dirs = vec![
        ("constraints", "../../spec/constraints/*.yaml"),
        ("logic", "../../spec/logic/**/*.yaml"),
        ("infrastructure", "../../spec/infrastructure/*.yaml"),
        ("events", "../../spec/events/*.yaml"),
        ("services", "../../spec/services/*.yaml"),
        ("frontends", "../../spec/frontends/*.yaml"),
    ];

    let mut data_map: HashMap<String, Vec<Value>> = HashMap::new();

    for (key, pattern) in spec_dirs {
        let mut parsed_files = Vec::new();
        for entry in glob(pattern).expect("Geçersiz glob deseni") {
            if let Ok(path) = entry {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(yaml_val) = serde_yaml::from_str::<Value>(&content) {
                        parsed_files.push(yaml_val);
                    }
                }
            }
        }
        data_map.insert(key.to_string(), parsed_files);
    }

    let dist_dir = "../../dist";
    fs::create_dir_all(dist_dir)?;
    fs::create_dir_all(format!("{}/topologies", dist_dir))?;

    // 1. AI Context Üret
    let mut ai_ctx = String::from("<vq_capital_context>\n");
    for (k, v) in &data_map {
        if v.is_empty() { continue; }
        ai_ctx.push_str(&format!("<{}>\n", k));
        ai_ctx.push_str(&serde_json::to_string(v)?);
        ai_ctx.push_str(&format!("\n</{}>\n", k));
    }
    ai_ctx.push_str("</vq_capital_context>");
    fs::write(format!("{}/.vq-context.md", dist_dir), ai_ctx)?;

    // 2. NATS Mesh (Mermaid) Üret - VQ-CAPITAL NATS_MESH FORMATI İÇİN GÜNCELLENDİ
    let mut mesh = String::from("# ⚡ VQ-Capital NATS JetStream Mesh\n\n```mermaid\ngraph TD\n");
    mesh.push_str("    classDef eventNode fill:#fef08a,stroke:#eab308,stroke-width:2px,color:#000,shape:hexagon;\n");
    mesh.push_str("    classDef serviceNode fill:#f1f5f9,stroke:#64748b,stroke-width:1px,color:#000;\n");

    if let Some(events) = data_map.get("events") {
        for e_node in events {
            if let Some(mesh_obj) = e_node.get("mesh").and_then(|m| m.get("streams")).and_then(|s| s.as_mapping()) {
                for (stream_key, stream_val) in mesh_obj {
                    let stream_name = stream_key.as_str().unwrap_or("unknown");
                    let e_id = stream_name.replace('.', "_");
                    
                    mesh.push_str(&format!("    {}{{{{{}}}}}\n", e_id, stream_name));
                    
                    if let Some(pub_val) = stream_val.get("publisher").and_then(|v| v.as_str()) {
                        mesh.push_str(&format!("    {}[{}]:::serviceNode ==>|Publishes| {}\n", pub_val.replace("-", "_"), pub_val, e_id));
                    }
                    if let Some(cons_val) = stream_val.get("consumers").and_then(|v| v.as_sequence()) {
                        for c in cons_val {
                            if let Some(c_str) = c.as_str() {
                                mesh.push_str(&format!("    {} -.->|Consumes| {}[{}]:::serviceNode\n", e_id, c_str.replace("-", "_"), c_str));
                            }
                        }
                    }
                }
            }
        }
    }
    mesh.push_str("```\n");
    fs::write(format!("{}/topologies/nats-mesh.md", dist_dir), mesh)?;

    // 3. İnsanlar için Markdown Üret
    let mut doc = String::from("# 🦅 VQ-Capital Enterprise Architecture\n\n*(AUTO-GENERATED VIA RUST)*\n\n## 🧩 Microservices\n| Service | Responsibility | Technology | Max Latency |\n|---|---|---|---|\n");
    if let Some(services) = data_map.get("services") {
        for s in services {
            if let Some(srv) = s.get("service") {
                let name = srv.get("name").and_then(|v| v.as_str()).unwrap_or("-");
                let resp = srv.get("responsibility").and_then(|v| v.as_str()).unwrap_or("-").replace('\n', " ");
                let tech = srv.get("technology").and_then(|v| v.as_str()).unwrap_or("-");
                let latency = srv.get("sla").and_then(|v| v.get("max_response_time_ms")).and_then(|v| v.as_i64()).map(|v| format!("{}ms", v)).unwrap_or("-".to_string());
                doc.push_str(&format!("| `{name}` | {resp} | `{tech}` | `{latency}` |\n"));
            }
        }
    }

    doc.push_str("\n## ⚙️ Infrastructure\n| Component | Type | Responsibility |\n|---|---|---|\n");
    if let Some(infra) = data_map.get("infrastructure") {
        for i in infra {
            if let Some(inf) = i.get("infrastructure") {
                let name = inf.get("name").and_then(|v| v.as_str()).unwrap_or("-");
                let t = inf.get("type").and_then(|v| v.as_str()).unwrap_or("-");
                let resp = inf.get("responsibility").and_then(|v| v.as_str()).unwrap_or("-").replace('\n', " ");
                doc.push_str(&format!("| `{name}` | `{t}` | {resp} |\n"));
            }
        }
    }

    fs::write(format!("{}/architecture.md", dist_dir), doc)?;

    info!("✅ All docs successfully generated via Rust!");
    Ok(())
}

fn run_linter(target_dir: &str, repo_name: &str, spec_dir: &str) -> Result<()> {
    info!("🔍 (Rust Engine) VQ-Capital Zero-Tolerance Linter Started for: {}", repo_name);
    
    let rules_path = Path::new(spec_dir).join("spec/constraints/linter-rules.yaml");
    if !rules_path.exists() {
        warn!("Linter rules file not found, skipping.");
        return Ok(());
    }

    let rules_content = fs::read_to_string(rules_path)?;
    let rules_yaml: Value = serde_yaml::from_str(&rules_content)?;
    
    let mut has_failure = false;

    if let Some(rules) = rules_yaml.get("linter_rules").and_then(|v| v.as_sequence()) {
        for rule in rules {
            let target_pattern = rule.get("target_repo").and_then(|v| v.as_str()).unwrap_or("*");
            
            if target_pattern != "*" && !glob_match(target_pattern, repo_name) {
                continue;
            }

            if let Some(match_files) = rule.get("match_files").and_then(|v| v.as_sequence()) {
                for pattern_val in match_files {
                    let pattern_str = pattern_val.as_str().unwrap_or("");
                    let full_pattern = format!("{}/{}", target_dir, pattern_str);

                    for entry in glob(&full_pattern).expect("Geçersiz linter glob deseni") {
                        if let Ok(file_path) = entry {
                            if !file_path.is_file() { continue; }
                            
                            let content = fs::read_to_string(&file_path).unwrap_or_default();
                            
                            if let Some(forbidden) = rule.get("forbidden_patterns").and_then(|v| v.as_sequence()) {
                                for f in forbidden {
                                    let reg_str = f.get("regex").and_then(|v| v.as_str()).unwrap_or("");
                                    let msg = f.get("message").and_then(|v| v.as_str()).unwrap_or("Violation");
                                    
                                    if let Ok(re) = Regex::new(reg_str) {
                                        if re.is_match(&content) {
                                            error!(file = ?file_path, reason = msg, "❌ ARCHITECTURAL VIOLATION");
                                            has_failure = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if has_failure {
        error!("🚨 PIPELINE BLOCKED. ARCHITECTURAL FAULTS DETECTED.");
        std::process::exit(1);
    } else {
        info!("✅ SPEC COMPLIANCE PASSED.");
    }

    Ok(())
}

fn glob_match(pattern: &str, target: &str) -> bool {
    let p = pattern.replace("*", ".*");
    if let Ok(re) = Regex::new(&format!("^{}$", p)) {
        re.is_match(target)
    } else {
        false
    }
}