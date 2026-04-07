use euroleague_scenarios::*;
use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Mutex;
use std::time::Instant;


// ============================================================================
// MAIN
// ============================================================================

/// Global accumulator for a position across ALL RED scenarios
struct GlobalPosAccum {
    total: u64,
    opponents: BTreeMap<Team, u64>,
    /// For each of the 27 remaining games (by index in remaining_games()):
    /// [home_wins, away_wins] — who won this game in scenarios where RED reached this position
    game_winner_home: Vec<u64>,
    game_winner_away: Vec<u64>,
    /// Per RED-scenario breakdown: (combo_desc, count)
    red_scenario_counts: BTreeMap<String, u64>,
}

impl GlobalPosAccum {
    fn new(n_all_games: usize) -> Self {
        Self {
            total: 0,
            opponents: BTreeMap::new(),
            game_winner_home: vec![0u64; n_all_games],
            game_winner_away: vec![0u64; n_all_games],
            red_scenario_counts: BTreeMap::new(),
        }
    }
}

fn main() {
    let start = Instant::now();

    let base = base_standings();
    let h2h = base_h2h();
    let games = remaining_games();
    let n_all_games = games.len();

    let red_game_indices: Vec<usize> = games.iter().enumerate()
        .filter(|(_, g)| g.home == Team::RED || g.away == Team::RED)
        .map(|(i, _)| i)
        .collect();

    let other_game_indices: Vec<usize> = games.iter().enumerate()
        .filter(|(_, g)| g.home != Team::RED && g.away != Team::RED)
        .map(|(i, _)| i)
        .collect();

    println!("RED mecevi: {:?}", red_game_indices.iter()
        .map(|&i| format!("{} vs {} (gc={})", games[i].home.code(), games[i].away.code(), games[i].gamecode))
        .collect::<Vec<_>>());
    println!("Ostali mecevi: {}", other_game_indices.len());

    // Global accumulators across ALL RED scenarios
    let global_accum: Mutex<BTreeMap<u32, GlobalPosAccum>> = Mutex::new(BTreeMap::new());
    let global_eliminated: Mutex<u64> = Mutex::new(0);
    let global_total: Mutex<u64> = Mutex::new(0);

    for red_combo in 0u32..8 {
        let red_bits: Vec<bool> = (0..red_game_indices.len())
            .map(|i| (red_combo >> i) & 1 == 1)
            .collect();

        let red_new_wins: u32 = red_bits.iter().filter(|&&b| b).count() as u32;
        let red_total_wins = 19 + red_new_wins;

        let combo_desc: String = red_game_indices.iter().zip(red_bits.iter())
            .map(|(&gi, &win)| {
                let g = &games[gi];
                let opp = if g.home == Team::RED { g.away } else { g.home };
                format!("{}{}", if win { "W" } else { "L" }, opp.code())
            })
            .collect::<Vec<_>>()
            .join("/");

        eprint!("\r  Obradjujem RED {} ({})...       ", red_total_wins, combo_desc);

        // Determine critical games for this RED scenario
        let mut team_remaining: HashMap<Team, u32> = HashMap::new();
        for &gi in &other_game_indices {
            let g = &games[gi];
            *team_remaining.entry(g.home).or_insert(0) += 1;
            *team_remaining.entry(g.away).or_insert(0) += 1;
        }

        let can_tie: BTreeSet<Team> = RELEVANT_TEAMS.iter()
            .filter(|&&t| {
                if t == Team::RED { return false; }
                let s = base.iter().find(|s| s.team == t).unwrap();
                let rem = team_remaining.get(&t).copied().unwrap_or(0);
                s.wins <= red_total_wins && s.wins + rem >= red_total_wins
            })
            .copied()
            .collect();

        let mut critical_indices: Vec<usize> = Vec::new();
        let mut irrelevant_indices: Vec<usize> = Vec::new();

        for &gi in &other_game_indices {
            let g = &games[gi];
            if can_tie.contains(&g.home) || can_tie.contains(&g.away) {
                critical_indices.push(gi);
            } else {
                irrelevant_indices.push(gi);
            }
        }

        let game_options: Vec<(usize, Vec<GameOutcome>)> = critical_indices.iter()
            .map(|&gi| {
                let g = &games[gi];
                if let Some(info) = &g.h2h_info {
                    let first_winner_wins = if info.first_winner == g.home {
                        GameOutcome::HomeWin
                    } else {
                        GameOutcome::AwayWin
                    };
                    if info.margin == 1 {
                        (gi, vec![first_winner_wins, GameOutcome::H2HLoserWinsBig])
                    } else {
                        (gi, vec![first_winner_wins, GameOutcome::H2HLoserWinsSmall, GameOutcome::H2HLoserWinsBig])
                    }
                } else {
                    (gi, vec![GameOutcome::HomeWin, GameOutcome::AwayWin])
                }
            })
            .collect();

        let actual_combos: u64 = if game_options.is_empty() { 1 } else {
            game_options.iter().map(|(_, opts)| opts.len() as u64).product()
        };

        *global_total.lock().unwrap() += actual_combos;

        // Fixed outcomes
        let mut fixed_outcomes: Vec<(usize, GameOutcome)> = Vec::new();
        for &gi in &irrelevant_indices {
            fixed_outcomes.push((gi, GameOutcome::HomeWin));
        }
        for (idx, &gi) in red_game_indices.iter().enumerate() {
            let g = &games[gi];
            let red_wins = red_bits[idx];
            let outcome = if g.home == Team::RED {
                if red_wins { GameOutcome::HomeWin } else { GameOutcome::AwayWin }
            } else {
                if red_wins { GameOutcome::AwayWin } else { GameOutcome::HomeWin }
            };
            fixed_outcomes.push((gi, outcome));
        }

        let radixes: Vec<usize> = game_options.iter().map(|(_, opts)| opts.len()).collect();
        let n_crit = game_options.len();

        let chunk_size: u64 = if actual_combos > 100_000 { 10_000 } else { 1 };
        let n_chunks = (actual_combos + chunk_size - 1) / chunk_size;

        let combo_desc_clone = combo_desc.clone();

        (0..n_chunks).into_par_iter().for_each(|chunk_idx| {
            let start_c = chunk_idx * chunk_size;
            let end_c = std::cmp::min(start_c + chunk_size, actual_combos);

            // Local accumulators
            let mut local_pos: BTreeMap<u32, (u64, BTreeMap<Team, u64>, Vec<u64>, Vec<u64>)> = BTreeMap::new();
            let mut local_eliminated: u64 = 0;

            for combo_num in start_c..end_c {
                let mut choices = vec![0usize; n_crit];
                let mut remaining = combo_num;
                for i in 0..n_crit {
                    let radix = radixes[i] as u64;
                    choices[i] = (remaining % radix) as usize;
                    remaining /= radix;
                }

                let mut outcomes = fixed_outcomes.clone();
                for i in 0..n_crit {
                    let (gi, ref opts) = game_options[i];
                    outcomes.push((gi, opts[choices[i]]));
                }

                let result = run_scenario(&outcomes, &games, &base, &h2h);

                if result.has_unresolved_red || result.red_min != result.red_max {
                    local_eliminated += 1;
                    continue;
                }

                let red_pos = result.red_min;
                if red_pos > 10 {
                    local_eliminated += 1;
                    continue;
                }

                let entry = local_pos.entry(red_pos).or_insert_with(|| {
                    (0, BTreeMap::new(), vec![0u64; n_all_games], vec![0u64; n_all_games])
                });
                entry.0 += 1;

                // Record opponent
                if let Some(opp_pos) = opponent_position(red_pos) {
                    let opp_team = result.standings[(opp_pos - 1) as usize];
                    *entry.1.entry(opp_team).or_insert(0) += 1;
                }

                // Record game winners for ALL games in this scenario
                for &(gi, outcome) in &outcomes {
                    let g = &games[gi];
                    let home_won = match outcome {
                        GameOutcome::HomeWin => true,
                        GameOutcome::AwayWin => false,
                        GameOutcome::H2HLoserWinsSmall | GameOutcome::H2HLoserWinsBig => {
                            let info = g.h2h_info.as_ref().unwrap();
                            info.first_loser == g.home
                        }
                    };
                    if home_won {
                        entry.2[gi] += 1;
                    } else {
                        entry.3[gi] += 1;
                    }
                }
            }

            // Merge into global
            {
                let mut ga = global_accum.lock().unwrap();
                for (pos, (count, opps, home_w, away_w)) in &local_pos {
                    let entry = ga.entry(*pos).or_insert_with(|| GlobalPosAccum::new(n_all_games));
                    entry.total += count;
                    for (&t, &c) in opps {
                        *entry.opponents.entry(t).or_insert(0) += c;
                    }
                    for i in 0..n_all_games {
                        entry.game_winner_home[i] += home_w[i];
                        entry.game_winner_away[i] += away_w[i];
                    }
                    *entry.red_scenario_counts.entry(combo_desc_clone.clone()).or_insert(0) += count;
                }
            }
            {
                *global_eliminated.lock().unwrap() += local_eliminated;
            }
        });
    }

    eprintln!("\r  Gotovo!                                    ");

    // ===== FINAL OUTPUT =====
    let data = global_accum.lock().unwrap();
    let elim = *global_eliminated.lock().unwrap();
    let total_all = *global_total.lock().unwrap();

    println!("\n{}", "=".repeat(68));
    println!("════════════════════════════════════════════════════════════════════");
    println!("  UKUPNI REZULTATI — Crvena zvezda, sve kombinacije");
    println!("  Ukupno scenarija: {}", total_all);
    println!("════════════════════════════════════════════════════════════════════");

    for (&pos, pd) in data.iter() {
        let total = pd.total;
        let pct_of_all = total as f64 / total_all as f64 * 100.0;
        let round_name = playoff_round_name(pos);

        println!("\n┌──────────────────────────────────────────────────────────────────");
        println!("│ POZICIJA {} ({}) — {} scenarija ({:.1}%)", pos, round_name, total, pct_of_all);
        println!("├──────────────────────────────────────────────────────────────────");

        // RED scenario breakdown
        println!("│");
        println!("│ RED rezultati:");
        let mut red_vec: Vec<(&String, &u64)> = pd.red_scenario_counts.iter().collect();
        red_vec.sort_by(|a, b| b.1.cmp(a.1));
        for (desc, count) in &red_vec {
            let pct = **count as f64 / total as f64 * 100.0;
            println!("│   {:<20} {:>5.1}%  ({} scenarija)", desc, pct, count);
        }

        // Game results
        println!("│");
        println!("│ Mecevi (samo oni koji uticu):");

        let mut game_info: Vec<(usize, u64, u64)> = Vec::new();
        for gi in 0..n_all_games {
            let hw = pd.game_winner_home[gi];
            let aw = pd.game_winner_away[gi];
            if hw + aw > 0 {
                game_info.push((gi, hw, aw));
            }
        }

        // Sort: 100% required first, then by home_win%
        game_info.sort_by(|a, b| {
            let a_pct = a.1 as f64 / (a.1 + a.2) as f64;
            let b_pct = b.1 as f64 / (b.1 + b.2) as f64;
            // Sort required (100% or 0%) first
            let a_required = a_pct >= 0.999 || a_pct <= 0.001;
            let b_required = b_pct >= 0.999 || b_pct <= 0.001;
            b_required.cmp(&a_required)
                .then(b_pct.partial_cmp(&a_pct).unwrap_or(std::cmp::Ordering::Equal))
        });

        for &(gi, hw, aw) in &game_info {
            let g = &games[gi];
            let total_g = hw + aw;
            let home_pct = hw as f64 / total_g as f64 * 100.0;

            // Skip RED's games (already shown above)
            if g.home == Team::RED || g.away == Team::RED {
                continue;
            }

            if home_pct >= 99.9 {
                println!("│   ✓ {} vs {} → {} W (100%)",
                    g.home.code(), g.away.code(), g.home.code());
            } else if home_pct <= 0.1 {
                println!("│   ✓ {} vs {} → {} W (100%)",
                    g.home.code(), g.away.code(), g.away.code());
            } else {
                println!("│   ~ {} vs {} → {} W {:.0}% / {} W {:.0}%",
                    g.home.code(), g.away.code(),
                    g.home.code(), home_pct,
                    g.away.code(), 100.0 - home_pct);
            }
        }

        // Opponent
        if !pd.opponents.is_empty() {
            println!("│");
            if let Some(opp_pos) = opponent_position(pos) {
                println!("│ Protivnik ({}. mesto):", opp_pos);
            } else if pos <= 2 {
                println!("│ Protivnik: pobednik plej-ina");
            }

            let mut opp_vec: Vec<(Team, u64)> = pd.opponents.iter()
                .map(|(&t, &c)| (t, c)).collect();
            opp_vec.sort_by(|a, b| b.1.cmp(&a.1));

            for (team, count) in &opp_vec {
                let pct = *count as f64 / total as f64 * 100.0;
                if pct >= 0.5 {
                    println!("│   {:<4} {:>5.1}%", team.code(), pct);
                }
            }
            let small: u64 = opp_vec.iter()
                .filter(|(_, c)| (*c as f64 / total as f64 * 100.0) < 0.5)
                .map(|(_, c)| *c).sum();
            if small > 0 {
                println!("│   Ostali {:.1}%", small as f64 / total as f64 * 100.0);
            }
        }

        println!("└──────────────────────────────────────────────────────────────────");
    }

    if elim > 0 {
        let pct = elim as f64 / total_all as f64 * 100.0;
        println!("\n┌──────────────────────────────────────────────────────────────────");
        println!("│ ELIMINISAN (ispod 10. mesta) — {} scenarija ({:.1}%)", elim, pct);
        println!("└──────────────────────────────────────────────────────────────────");
    }

    let elapsed = start.elapsed();
    println!("\nUkupno vreme: {:.2}s", elapsed.as_secs_f64());
}
