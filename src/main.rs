mod ast;
mod codegen;
mod config;
mod lexer;
mod packager;
mod parser;
mod scratch;

use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "snap")]
#[command(author = "Snap Contributors")]
#[command(version = env!("CARGO_PKG_VERSION").is_empty().then(|| "0.1.1").unwrap_or(env!("CARGO_PKG_VERSION")))]
#[command(
    about = "A programming language that compiles to Scratch (.sb3) projects",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Snap project
    New {
        /// Name of the project to create
        name: String,
        /// Author name (optional)
        #[arg(short, long)]
        author: Option<String>,
    },
    /// Build the current project
    Build {
        /// Path to the project directory (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,
        /// Output file path (defaults to <project_name>.sb3)
        #[arg(short, long)]
        output: Option<String>,
        /// Enable verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Build and run the project (opens in Scratch)
    Run {
        /// Path to the project directory (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,
    },
    /// Check the project for errors without building
    Check {
        /// Path to the project directory (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,
    },
    /// Initialize a new project in the current directory
    Init {
        /// Author name (optional)
        #[arg(short, long)]
        author: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New { name, author } => cmd_new(&name, author),
        Commands::Build {
            path,
            output,
            verbose,
        } => cmd_build(&path, output, verbose),
        Commands::Run { path } => cmd_run(&path),
        Commands::Check { path } => cmd_check(&path),
        Commands::Init { author } => cmd_init(author),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn cmd_new(name: &str, author: Option<String>) -> Result<(), String> {
    let project_path = Path::new(name);

    if project_path.exists() {
        return Err(format!("Directory '{}' already exists", name));
    }

    // Create directory structure
    fs::create_dir_all(project_path.join("src"))
        .map_err(|e| format!("Failed to create directory: {}", e))?;

    // Create config.toml
    let author_line = match &author {
        Some(a) => format!("author = \"{}\"", a),
        None => "# author = \"Your Name\"".to_string(),
    };
    let config_content = format!(
        r#"[project]
name = "{}"
{}
"#,
        name, author_line
    );
    fs::write(project_path.join("config.toml"), config_content)
        .map_err(|e| format!("Failed to create config.toml: {}", e))?;

    // Create main.sp with a simple example
    let main_content = r#"// Welcome to Snap!
// This is your main source file.

new Sprite("Sprite1") {
    implements Code {
        on GreenFlag {
            looks::SayTimed("Hello, Snap!", units::Sec(2));
        }
    }
}
"#;
    fs::write(project_path.join("src").join("main.sp"), main_content)
        .map_err(|e| format!("Failed to create main.sp: {}", e))?;

    // Create .gitignore
    let gitignore_content = "# Build output\ntarget/*.sb3\n";
    fs::write(project_path.join(".gitignore"), gitignore_content)
        .map_err(|e| format!("Failed to create .gitignore: {}", e))?;

    println!("{} `{}` project", "Created".green().bold(), name);
    println!("\nTo get started:");
    println!("  {} {}", "cd".cyan(), name);
    println!("  {} build", "snap".cyan());

    Ok(())
}

fn cmd_init(author: Option<String>) -> Result<(), String> {
    let current_dir =
        std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;

    let name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("my_project");

    // Check if already initialized
    if current_dir.join("config.toml").exists() {
        return Err("Project already initialized (config.toml exists)".to_string());
    }

    // Create src directory
    fs::create_dir_all(current_dir.join("src"))
        .map_err(|e| format!("Failed to create src directory: {}", e))?;

    // Create config.toml
    let author_line = match &author {
        Some(a) => format!("author = \"{}\"", a),
        None => "# author = \"Your Name\"".to_string(),
    };
    let config_content = format!(
        r#"[project]
name = "{}"
{}
"#,
        name, author_line
    );
    fs::write(current_dir.join("config.toml"), config_content)
        .map_err(|e| format!("Failed to create config.toml: {}", e))?;

    // Create main.sp if it doesn't exist
    let main_path = current_dir.join("src").join("main.sp");
    if !main_path.exists() {
        let main_content = r#"// Welcome to Snap!
// This is your main source file.

new Sprite("Sprite1") {
    implements Code {
        on GreenFlag {
            looks::SayTimed("Hello, Snap!", units::Sec(2));
        }
    }
}
"#;
        fs::write(&main_path, main_content)
            .map_err(|e| format!("Failed to create main.sp: {}", e))?;
    }

    println!(
        "{} Snap project in current directory",
        "Initialized".green().bold()
    );

    Ok(())
}

fn cmd_build(path: &str, output: Option<String>, verbose: bool) -> Result<(), String> {
    let project_path = Path::new(path);

    // Load config
    let config_path = project_path.join("config.toml");
    let config = config::load_config(&config_path)?;

    if verbose {
        println!(
            "{} project: {}",
            "Compiling".cyan().bold(),
            config.project.name
        );
    }

    // Read main source file
    let main_sp_path = project_path.join("src").join("main.sp");
    let source = fs::read_to_string(&main_sp_path)
        .map_err(|e| format!("Failed to read {}: {}", main_sp_path.display(), e))?;

    // Compile
    let program = compile_source(&source, project_path)?;

    if verbose {
        println!("  {} {} sprite(s)", "Found".green(), program.sprites.len());
        if program.stage.is_some() {
            println!("  {} stage definition", "Found".green());
        }
    }

    // Code generation
    let scratch_project = codegen::generate(&config, &program);

    // Determine output path
    let output_path = match output {
        Some(o) => Path::new(&o).to_path_buf(),
        None => {
            let target_dir = project_path.join("target");
            fs::create_dir_all(&target_dir)
                .map_err(|e| format!("Failed to create target directory: {}", e))?;
            target_dir.join(format!("{}.sb3", config.project.name))
        }
    };

    // Package into .sb3
    packager::package(&scratch_project, &output_path)?;

    println!(
        "{} {} ({})",
        "Finished".green().bold(),
        config.project.name,
        output_path.display()
    );

    Ok(())
}

fn cmd_run(path: &str) -> Result<(), String> {
    // First build the project
    cmd_build(path, None, false)?;

    let project_path = Path::new(path);
    let config_path = project_path.join("config.toml");
    let config = config::load_config(&config_path)?;
    let sb3_path = project_path
        .join("target")
        .join(format!("{}.sb3", config.project.name));

    // Try to open in browser with Scratch
    let scratch_url = "https://scratch.mit.edu/projects/editor/?tutorial=getStarted".to_string();

    println!("{} Opening Scratch editor...", "Info:".cyan().bold());
    println!("  Load your project by clicking File > Load from your computer");
    println!("  Project file: {}", sb3_path.display());

    // Try to open the URL
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(&scratch_url).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", &scratch_url])
            .spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open")
            .arg(&scratch_url)
            .spawn();
    }

    Ok(())
}

fn cmd_check(path: &str) -> Result<(), String> {
    let project_path = Path::new(path);

    // Load config
    let config_path = project_path.join("config.toml");
    let config = config::load_config(&config_path)?;

    println!("{} {}", "Checking".cyan().bold(), config.project.name);

    // Read main source file
    let main_sp_path = project_path.join("src").join("main.sp");
    let source = fs::read_to_string(&main_sp_path)
        .map_err(|e| format!("Failed to read {}: {}", main_sp_path.display(), e))?;

    // Try to compile (but don't generate output)
    let program = compile_source(&source, project_path)?;

    println!(
        "{} {} sprite(s), {} global(s)",
        "Found:".green(),
        program.sprites.len(),
        program.globals.len()
    );

    if program.stage.is_some() {
        println!("  {} Stage definition", "+".green());
    }

    for sprite in &program.sprites {
        let handler_count = sprite
            .code
            .as_ref()
            .map(|c| c.event_handlers.len())
            .unwrap_or(0);
        let fn_count = sprite.code.as_ref().map(|c| c.functions.len()).unwrap_or(0);
        println!(
            "  {} Sprite '{}': {} event(s), {} function(s)",
            "+".green(),
            sprite.name,
            handler_count,
            fn_count
        );
    }

    println!("\n{} No errors found!", "Success:".green().bold());

    Ok(())
}

fn compile_source(source: &str, project_path: &Path) -> Result<ast::Program, String> {
    // Lexical analysis
    let tokens = lexer::tokenize(source)?;

    // Parsing
    let mut program = parser::parse(tokens)?;

    // Handle imports
    let mut all_sprites = Vec::new();
    let mut all_globals = Vec::new();

    for import in &program.imports {
        let import_path = project_path.join(&import.path);
        let import_source = fs::read_to_string(&import_path)
            .map_err(|e| format!("Error reading import {}: {}", import.path, e))?;

        let imported_program = compile_source(&import_source, project_path)?;

        // Merge imported content
        all_sprites.extend(imported_program.sprites);
        all_globals.extend(imported_program.globals);

        // If imported file has a stage definition, warn (only one stage allowed)
        if imported_program.stage.is_some() && program.stage.is_some() {
            return Err("Multiple Stage definitions found across files".to_string());
        }
        if imported_program.stage.is_some() {
            program.stage = imported_program.stage;
        }
    }

    // Add sprites and globals from the main file
    all_sprites.extend(program.sprites);
    all_globals.extend(program.globals);

    program.sprites = all_sprites;
    program.globals = all_globals;

    Ok(program)
}
