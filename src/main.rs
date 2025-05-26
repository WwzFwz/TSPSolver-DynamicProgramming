use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

const INF: i32 = i32::MAX / 2;

struct TSPSolver {
    n: usize,
    dist: Vec<Vec<i32>>,
    dp: HashMap<(usize, usize), i32>,
    progress_bar: Option<ProgressBar>,
    total_states: usize,
    computed_states: usize,
}

impl TSPSolver {
    fn new(distances: Vec<Vec<i32>>) -> Self {
        let n = distances.len();
        let total_states = n * (1 << n);
        
        TSPSolver {
            n,
            dist: distances,
            dp: HashMap::new(),
            progress_bar: None,
            total_states,
            computed_states: 0,
        }
    }

    fn set_progress_bar(&mut self, pb: ProgressBar) {
        self.progress_bar = Some(pb);
    }

    fn solve(&mut self) -> (i32, Vec<usize>) {
        if self.n <= 1 {
            return (0, vec![0]);
        }

        println!("{}", "🔍 Solving TSP using Dynamic Programming...".bright_cyan());
        
        // Solve TSP using dynamic programming with bitmask
        let min_cost = self.tsp_dp(1, 0); // Start from city 0, visited only city 0
        
        if let Some(ref pb) = self.progress_bar {
            pb.set_message("Reconstructing optimal path...");
        }
        
        let path = self.reconstruct_path();
        
        if let Some(ref pb) = self.progress_bar {
            pb.finish_with_message("✅ TSP solved successfully!");
        }
        
        (min_cost, path)
    }

    fn tsp_dp(&mut self, mask: usize, pos: usize) -> i32 {
        // Update progress
        self.computed_states += 1;
        if let Some(ref pb) = self.progress_bar {
            if self.computed_states % 100 == 0 {
                let progress = (self.computed_states as f64 / self.total_states as f64 * 100.0) as u64;
                pb.set_position(progress.min(95)); // Keep some room for path reconstruction
            }
        }

        // Base case: if all cities are visited, return cost to start city
        if mask == (1 << self.n) - 1 {
            return self.dist[pos][0];
        }

        // Check if already computed
        if let Some(&result) = self.dp.get(&(mask, pos)) {
            return result;
        }

        let mut ans = INF;

        // Try to go to every city that hasn't been visited
        for city in 0..self.n {
            if (mask & (1 << city)) == 0 { // City not visited
                let new_mask = mask | (1 << city);
                let cost = self.dist[pos][city] + self.tsp_dp(new_mask, city);
                ans = ans.min(cost);
            }
        }

        self.dp.insert((mask, pos), ans);
        ans
    }

    fn reconstruct_path(&mut self) -> Vec<usize> {
        let mut path = Vec::new();
        let mut mask = 1; // Start with only city 0 visited
        let mut pos = 0;
        
        path.push(0);

        while mask != (1 << self.n) - 1 {
            let mut next_city = 0;
            let mut min_cost = INF;

            for city in 0..self.n {
                if (mask & (1 << city)) == 0 { // City not visited
                    let new_mask = mask | (1 << city);
                    
                    // Get the DP value, compute if not exists
                    let dp_value = if let Some(&val) = self.dp.get(&(new_mask, city)) {
                        val
                    } else {
                        self.tsp_dp(new_mask, city)
                    };
                    
                    let cost = self.dist[pos][city] + dp_value;
                    
                    if cost < min_cost {
                        min_cost = cost;
                        next_city = city;
                    }
                }
            }

            path.push(next_city);
            mask |= 1 << next_city;
            pos = next_city;
        }

        path.push(0); // Return to start
        path
    }
}

fn center_text(text: &str, width: usize) -> String {
    let padding = if text.len() < width {
        (width - text.len()) / 2
    } else {
        0
    };
    format!("{}{}", " ".repeat(padding), text)
}

fn print_banner() {
    let width = 70;
    println!("{}", "═".repeat(width).bright_cyan());
    println!(
        "{}",
        center_text("🚀 TRAVELING SALESMAN PROBLEM SOLVER 🚀", width)
            .bold()
            .yellow()
    );
    println!(
        "{}",
        center_text("Dynamic Programming with Bitmask Implementation", width)
            .italic()
            .white()
    );
    println!(
        "{}",
        center_text("IF2211 STIMA - Tugas Pemrograman Dinamis", width)
            .dimmed()
            .white()
    );
    println!("{}", "═".repeat(width).bright_cyan());
    println!();
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    println!("{}", "📊 Distance Matrix:".bright_green().bold());
    println!();
    
    // Header
    print!("     ");
    for i in 0..matrix.len() {
        print!("{:>8}", format!("City{}", i).bright_blue());
    }
    println!();
    
    // Separator
    print!("   ");
    for _ in 0..=matrix.len() {
        print!("--------");
    }
    println!();
    
    // Matrix rows
    for (i, row) in matrix.iter().enumerate() {
        print!("{:>5}|", format!("City{}", i).bright_blue());
        for &val in row {
            if val == INF {
                print!("{:>8}", "∞".red());
            } else if val == 0 {
                print!("{:>8}", "0".dimmed());
            } else {
                print!("{:>8}", val.to_string().white());
            }
        }
        println!();
    }
    println!();
}

fn parse_input(content: &str) -> Result<Vec<Vec<i32>>, String> {
    let lines: Vec<&str> = content.trim().lines().collect();
    
    if lines.is_empty() {
        return Err("Empty input file".to_string());
    }

    // First line should contain number of cities
    let n: usize = lines[0].trim().parse()
        .map_err(|_| "Invalid number of cities")?;

    if n == 0 {
        return Err("Number of cities must be greater than 0".to_string());
    }

    let mut distances = vec![vec![0; n]; n];

    // Format 1: Adjacency matrix (n+1 lines total)
    if lines.len() == n + 1 {
        for i in 0..n {
            let row: Result<Vec<i32>, _> = lines[i + 1]
                .trim()
                .split_whitespace()
                .map(|s| {
                    let s = s.trim().to_uppercase();
                    if s == "INF" || s == "∞" {
                        Ok(INF)
                    } else {
                        s.parse::<i32>()
                    }
                })
                .collect();
            
            match row {
                Ok(values) => {
                    if values.len() != n {
                        return Err(format!("Row {} has {} values, expected {}", i, values.len(), n));
                    }
                    distances[i] = values;
                }
                Err(_) => return Err(format!("Invalid number in row {}", i + 1)),
            }
        }
    }
    // Format 2: Edge list format
    else {
        // Initialize with infinity
        for i in 0..n {
            for j in 0..n {
                distances[i][j] = if i == j { 0 } else { INF };
            }
        }

        // Parse edges
        for (line_num, line) in lines.iter().skip(1).enumerate() {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() != 3 {
                return Err(format!("Line {}: Expected 3 values (from to weight)", line_num + 2));
            }

            let from: usize = parts[0].parse()
                .map_err(|_| format!("Line {}: Invalid 'from' city", line_num + 2))?;
            let to: usize = parts[1].parse()
                .map_err(|_| format!("Line {}: Invalid 'to' city", line_num + 2))?;
            let weight: i32 = parts[2].parse()
                .map_err(|_| format!("Line {}: Invalid weight", line_num + 2))?;

            if from >= n || to >= n {
                return Err(format!("Line {}: City index out of range", line_num + 2));
            }

            distances[from][to] = weight;
            distances[to][from] = weight; // Assume undirected graph
        }
    }

    Ok(distances)
}

fn format_path(path: &[usize]) -> String {
    path.iter()
        .map(|&i| format!("City{}", i))
        .collect::<Vec<_>>()
        .join(" → ")
}

fn print_solution(cost: i32, path: &[usize], elapsed: std::time::Duration, solver: &TSPSolver) {
    let width = 70;
    println!();
    println!(
        "{}",
        center_text("✨ SOLUTION FOUND! ✨", width)
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        center_text("═".repeat(width).as_str(), width).bright_cyan()
    );

    if cost >= INF {
        println!(
            "{}",
            center_text("❌ No valid tour found!", width).red().bold()
        );
        println!(
            "{}",
            center_text("The graph might not be connected.", width).yellow()
        );
    } else {
        println!(
            "{}",
            center_text(&format!("🎯 Minimum Cost: {}", cost), width)
                .bright_yellow()
                .bold()
        );
        println!(
            "{}",
            center_text(&format!("🗺️  Optimal Path: {}", format_path(path)), width).bright_white()
        );
        println!(
            "{}",
            center_text(&format!("⏱️  Computation Time: {:.3?}", elapsed), width).white()
        );
        println!(
            "{}",
            center_text(&format!("🏙️  Cities Visited: {}", path.len() - 1), width).white()
        );
        println!(
            "{}",
            center_text(&format!("🔢 DP States Computed: {}", solver.computed_states), width).dimmed()
        );
    }

    println!(
        "{}",
        center_text("═".repeat(width).as_str(), width).bright_cyan()
    );

    // Detailed route for small instances
    if path.len() <= 12 && cost < INF {
        println!();
        println!("{}", "📍 Detailed Route:".bright_magenta().bold());
        for i in 0..path.len() - 1 {
            let from = path[i];
            let to = path[i + 1];
            let distance = solver.dist[from][to];
            println!(
                "   Step {}: {} → {} (distance: {})",
                format!("{:2}", i + 1).bright_blue(),
                format!("City{}", from).bright_cyan(),
                format!("City{}", to).bright_cyan(),
                distance.to_string().yellow()
            );
        }
        println!();
    }
}

fn get_input_method() -> Result<bool, Box<dyn std::error::Error>> {
    let options = vec![
        "📁 Load from file",
        "⌨️  Enter matrix manually",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("How would you like to input the distance matrix?")
        .items(&options)
        .default(0)
        .interact()?;

    Ok(selection == 0)
}

fn get_file_path() -> Result<String, Box<dyn std::error::Error>> {
    loop {
        let file_path: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter the path to your matrix file")
            .interact_text()?;

        if Path::new(&file_path).exists() {
            return Ok(file_path);
        } else {
            println!(
                "{}",
                "❌ File not found! Please check the path and try again.".red()
            );
            println!();
        }
    }
}

fn create_progress_bar() -> ProgressBar {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}% ({eta}) {msg}",
            )
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );
    pb
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_banner();
    
    println!("🎮 Welcome to the advanced TSP Solver!");
    println!("This program solves the Traveling Salesman Problem using Dynamic Programming with Bitmask.\n");
    
    println!("📋 Instructions:");
    println!("  • Matrix format: n (first line), then n×n distance matrix");
    println!("  • Edge format: n (first line), then edges as 'from to weight'");
    println!("  • Use 0 for diagonal elements (city to itself)");
    println!("  • Use INF or ∞ for unreachable paths");
    println!("  • Cities are numbered from 0 to n-1\n");

    let distances = if std::env::args().len() > 1 {
        // Command line argument provided
        let file_path = std::env::args().nth(1).unwrap();
        println!("📂 Reading from file: {}", file_path.bright_blue());
        
        let content = fs::read_to_string(&file_path)
            .map_err(|e| format!("Error reading file '{}': {}", file_path, e))?;
        
        parse_input(&content)
            .map_err(|e| format!("Error parsing input: {}", e))?
    } else {
        // Interactive mode
        if get_input_method()? {
            let file_path = get_file_path()?;
            println!("📂 Reading matrix file...");
            
            let content = fs::read_to_string(&file_path)
                .map_err(|e| format!("Error reading file: {}", e))?;
            
            parse_input(&content)
                .map_err(|e| format!("Error parsing input: {}", e))?
        } else {
            return Err("Manual input not implemented yet".into());
        }
    };

    println!("✅ Matrix loaded successfully!\n");
    print_matrix(&distances);

    let n = distances.len();
    if n > 20 {
        println!(
            "{}",
            format!("⚠️  Large matrix detected ({} cities). This will take exponential time!", n)
                .yellow()
                .bold()
        );
        let continue_anyway = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Continue anyway? (Not recommended for n > 20)")
            .default(false)
            .interact()?;

        if !continue_anyway {
            println!("🛑 Operation cancelled.");
            return Ok(());
        }
    } else if n > 15 {
        println!(
            "{}",
            format!("⚠️  Medium-large matrix ({} cities). This may take some time.", n)
                .yellow()
        );
    }

    let pb = create_progress_bar();
    let start_time = Instant::now();
    
    let mut solver = TSPSolver::new(distances);
    solver.set_progress_bar(pb);
    
    let (min_cost, optimal_path) = solver.solve();
    let elapsed = start_time.elapsed();

    print_solution(min_cost, &optimal_path, elapsed, &solver);
    
    println!();
    println!("🙏 Thank you for using TSP Solver!");
    println!("💡 Tip: For better performance with large graphs, consider approximation algorithms.");
    
    Ok(())
}