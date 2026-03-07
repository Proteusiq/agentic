use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::Command;

const AGENTS_TEMPLATE: &str = include_str!("../../AGENTS.md");
const SKILL_TEMPLATE: &str = include_str!("../../SKILL.template.md");

#[derive(Parser, Debug)]
#[command(
    name = "scaffold",
    about = "Initialize AGENTS.md and SKILL.md in a repository",
    version
)]
struct Args {
    /// Target directory (default: current directory)
    #[arg(short, long, default_value = ".")]
    dir: PathBuf,

    /// GitHub organization (default: auto-detect from git remote)
    #[arg(short, long)]
    org: Option<String>,

    /// Only create SKILL.md (use existing AGENTS.md)
    #[arg(short, long)]
    skill_only: bool,

    /// Only create AGENTS.md (skip SKILL.md)
    #[arg(short, long)]
    agents_only: bool,

    /// Create symlink to embedded AGENTS.md instead of copying
    #[arg(short, long)]
    link: bool,

    /// Project name (default: auto-detect from directory or git remote)
    project_name: Option<String>,
}

fn detect_git_remote(dir: &Path) -> Option<String> {
    Command::new("git")
        .args(["remote", "get-url", "origin"])
        .current_dir(dir)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn parse_org_from_remote(remote: &str) -> String {
    // Handle both SSH and HTTPS URLs:
    // git@github.com:org/repo.git
    // https://github.com/org/repo.git
    let remote = remote.trim_end_matches(".git");

    if remote.contains("github.com/") {
        // HTTPS format: https://github.com/org/repo
        if let Some(pos) = remote.find("github.com/") {
            let path = &remote[pos + 11..];
            return path.split('/').next().unwrap_or("ORG").to_string();
        }
    }

    if let Some(pos) = remote.rfind(':') {
        // SSH format: git@github.com:org/repo
        let path = &remote[pos + 1..];
        return path.split('/').next().unwrap_or("ORG").to_string();
    }

    "ORG".to_string()
}

fn parse_repo_from_remote(remote: &str) -> String {
    let remote = remote.trim_end_matches(".git");
    remote.rsplit('/').next().unwrap_or("repo").to_string()
}

fn detect_org(dir: &Path) -> String {
    detect_git_remote(dir)
        .map(|r| parse_org_from_remote(&r))
        .unwrap_or_else(|| "ORG".to_string())
}

fn detect_repo(dir: &Path) -> String {
    detect_git_remote(dir)
        .map(|r| parse_repo_from_remote(&r))
        .unwrap_or_else(|| {
            dir.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("project")
                .to_string()
        })
}

fn to_upper_snake(s: &str) -> String {
    s.to_uppercase().replace('-', "_")
}

fn sanitize_name(name: &str) -> Result<String> {
    // Reject path traversal attempts and shell metacharacters
    if name.contains("..")
        || name.contains('/')
        || name.contains('\\')
        || name.contains('\0')
        || name.contains('`')
        || name.contains('$')
        || name.contains(';')
        || name.contains('|')
        || name.contains('&')
        || name.contains('\n')
        || name.contains('\r')
    {
        anyhow::bail!("Invalid project name: contains forbidden characters");
    }

    // Ensure name is not empty and reasonable length
    let name = name.trim();
    if name.is_empty() {
        anyhow::bail!("Project name cannot be empty");
    }
    if name.len() > 64 {
        anyhow::bail!("Project name too long (max 64 characters)");
    }

    Ok(name.to_string())
}

fn create_agents(target_dir: &Path, link_mode: bool) -> Result<()> {
    let agents_file = target_dir.join("AGENTS.md");

    if agents_file.exists() {
        println!("AGENTS.md already exists, skipping");
        return Ok(());
    }

    if link_mode {
        // For link mode, we write to a cache location and symlink
        let cache_dir = dirs::cache_dir()
            .context("Cannot determine cache directory (set XDG_CACHE_HOME or HOME)")?
            .join("agentic");
        fs::create_dir_all(&cache_dir)?;

        // Ensure we own the cache directory (prevent symlink attacks in shared dirs)
        let metadata = fs::metadata(&cache_dir)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            if metadata.uid() != unsafe { libc::getuid() } {
                anyhow::bail!(
                    "Cache directory {:?} is not owned by current user",
                    cache_dir
                );
            }
        }

        let cached_agents = cache_dir.join("AGENTS.md");
        fs::write(&cached_agents, AGENTS_TEMPLATE)?;

        symlink(&cached_agents, &agents_file)
            .with_context(|| format!("Failed to create symlink: {:?}", agents_file))?;

        println!("Linked AGENTS.md -> {:?}", cached_agents);
    } else {
        fs::write(&agents_file, AGENTS_TEMPLATE)?;
        println!("Created AGENTS.md");
    }

    Ok(())
}

fn create_skill(target_dir: &Path, project_name: &str, org: &str, repo: &str) -> Result<()> {
    let skill_file = target_dir.join("SKILL.md");

    if skill_file.exists() {
        println!("SKILL.md already exists, skipping");
        return Ok(());
    }

    let project_upper = to_upper_snake(project_name);

    let content = SKILL_TEMPLATE
        .replace("{{PROJECT_NAME}}", project_name)
        .replace("{{PROJECT_NAME_UPPER}}", &project_upper)
        .replace("{{GITHUB_ORG}}", org)
        .replace("{{REPO_NAME}}", repo)
        .replace(
            "{{SHORT_DESCRIPTION}}",
            &format!("Architecture, domain rules, and conventions for {project_name}"),
        )
        .replace("{{LONGER_DESCRIPTION}}", "TODO: Add project description")
        .replace(
            "{{PRODUCTION_URL_OR_DESCRIPTION}}",
            "TODO: Add production URL",
        )
        .replace("{{SERVICE_NAME}}", "core")
        .replace("{{RESOURCE}}", "items")
        .replace("{{MAIN_FUNCTION}}", "process")
        .replace("{{MODEL_NAME}}", "Item")
        .replace("{{DOMAIN_AREA_1}}", "Core Business Rules")
        .replace("{{DOMAIN_AREA_2}}", "Data Validation")
        .replace(
            "{{DESCRIBE_BUSINESS_RULES}}",
            "TODO: Document business rules",
        )
        .replace("{{DESCRIBE_MORE_RULES}}", "TODO: Document more rules")
        .replace("{{EXTERNAL_API}}", "EXTERNAL")
        .replace("{{ISSUE_1_TITLE}}", "Known Issue 1")
        .replace("{{ISSUE_2_TITLE}}", "Known Issue 2")
        .replace("{{DESCRIBE_ISSUE_AND_SOLUTION}}", "TODO: Document issue")
        .replace("{{N}}", "1")
        .replace("{{DESCRIPTION}}", "TODO");

    fs::write(&skill_file, content)?;
    println!("Created SKILL.md (edit to add project-specific details)");

    Ok(())
}

fn print_skill_link_instructions(project_name: &str, skill_path: &Path) {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    let opencode_dir = home.join(".config/opencode/skills").join(project_name);
    let claude_dir = home.join(".claude/skills").join(project_name);
    let skill_path = skill_path.display();

    println!();
    println!("To make this skill available globally, run:");
    println!();
    println!("  # OpenCode");
    println!("  mkdir -p {}", opencode_dir.display());
    println!(
        "  ln -sfn {} {}/SKILL.md",
        skill_path,
        opencode_dir.display()
    );
    println!();
    println!("  # Claude Code");
    println!("  mkdir -p {}", claude_dir.display());
    println!("  ln -sfn {} {}/SKILL.md", skill_path, claude_dir.display());
}

fn main() -> Result<()> {
    let args = Args::parse();

    let target_dir = args
        .dir
        .canonicalize()
        .with_context(|| format!("Cannot access directory: {:?}", args.dir))?;

    let repo = sanitize_name(&detect_repo(&target_dir))?;
    let project_name = match args.project_name {
        Some(name) => sanitize_name(&name)?,
        None => repo.clone(),
    };
    let org = match args.org {
        Some(o) => sanitize_name(&o)?,
        None => sanitize_name(&detect_org(&target_dir))?,
    };

    println!("Scaffolding agent docs for: {org}/{repo} (project: {project_name})");
    println!();

    if !args.skill_only {
        create_agents(&target_dir, args.link)?;
    }

    if !args.agents_only {
        create_skill(&target_dir, &project_name, &org, &repo)?;
        print_skill_link_instructions(&project_name, &target_dir.join("SKILL.md"));
    }

    println!();
    println!("Done! Edit the files to add project-specific details.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_org_ssh() {
        assert_eq!(
            parse_org_from_remote("git@github.com:Proteusiq/agentic.git"),
            "Proteusiq"
        );
    }

    #[test]
    fn test_parse_org_https() {
        assert_eq!(
            parse_org_from_remote("https://github.com/Proteusiq/agentic.git"),
            "Proteusiq"
        );
    }

    #[test]
    fn test_parse_repo() {
        assert_eq!(
            parse_repo_from_remote("git@github.com:Proteusiq/agentic.git"),
            "agentic"
        );
        assert_eq!(
            parse_repo_from_remote("https://github.com/Proteusiq/agentic"),
            "agentic"
        );
    }

    #[test]
    fn test_to_upper_snake() {
        assert_eq!(to_upper_snake("my-project"), "MY_PROJECT");
        assert_eq!(to_upper_snake("myproject"), "MYPROJECT");
    }

    #[test]
    fn test_sanitize_name_valid() {
        assert!(sanitize_name("my-project").is_ok());
        assert!(sanitize_name("myproject123").is_ok());
        assert!(sanitize_name("a").is_ok());
    }

    #[test]
    fn test_sanitize_name_path_traversal() {
        assert!(sanitize_name("../etc/passwd").is_err());
        assert!(sanitize_name("foo/../bar").is_err());
        assert!(sanitize_name("foo/bar").is_err());
        assert!(sanitize_name("foo\\bar").is_err());
    }

    #[test]
    fn test_sanitize_name_shell_injection() {
        assert!(sanitize_name("foo;rm -rf /").is_err());
        assert!(sanitize_name("foo`id`").is_err());
        assert!(sanitize_name("foo$(whoami)").is_err());
        assert!(sanitize_name("foo|cat /etc/passwd").is_err());
        assert!(sanitize_name("foo&& echo pwned").is_err());
    }

    #[test]
    fn test_sanitize_name_empty_and_long() {
        assert!(sanitize_name("").is_err());
        assert!(sanitize_name("   ").is_err());
        assert!(sanitize_name(&"a".repeat(65)).is_err());
        assert!(sanitize_name(&"a".repeat(64)).is_ok());
    }

    #[test]
    fn test_sanitize_name_newlines() {
        assert!(sanitize_name("foo\nbar").is_err());
        assert!(sanitize_name("foo\rbar").is_err());
    }
}
