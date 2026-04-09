use euroleague_scenarios::*;
use axum::{
    extract::Json,
    response::Html,
    routing::{get, post},
    Router,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Mutex;

// ============================================================================
// API types
// ============================================================================

#[derive(Deserialize)]
struct SimulateRequest {
    /// Map of game index -> chosen outcome index (from game_options).
    /// Games not in this map will be enumerated.
    locked: HashMap<usize, usize>,
}

#[derive(Serialize)]
struct GameInfo {
    index: usize,
    home: String,
    away: String,
    gamecode: u32,
    is_red_game: bool,
    options: Vec<String>,
    /// true if this game involves a team that could potentially tie RED
    relevant: bool,
    /// First game result for H2H tiebreaker games, e.g. "OLY W +12 vs MAD"
    h2h_first_game: Option<String>,
}

#[derive(Serialize)]
struct PositionResult {
    position: u32,
    round_name: String,
    count: u64,
    pct: f64,
    opponents: Vec<OpponentInfo>,
    game_results: Vec<GameResultInfo>,
}

#[derive(Serialize)]
struct OpponentInfo {
    team: String,
    pct: f64,
}

#[derive(Serialize)]
struct GameResultInfo {
    index: usize,
    home: String,
    away: String,
    home_pct: f64,
    required_winner: Option<String>,
}

#[derive(Serialize)]
struct GameImpact {
    index: usize,
    home: String,
    away: String,
    options: Vec<OptionImpact>,
}

#[derive(Serialize)]
struct OptionImpact {
    label: String,
    expected_pos: f64,
    delta: f64,
}

#[derive(Serialize)]
struct SimulateResponse {
    total_scenarios: u64,
    elapsed_ms: u64,
    positions: Vec<PositionResult>,
    eliminated_count: u64,
    eliminated_pct: f64,
    expected_position: f64,
    game_impacts: Vec<GameImpact>,
}

// ============================================================================
// Compute relevant games (depends on which RED games are locked)
// ============================================================================

fn describe_option(game: &RemainingGame, opt_idx: usize) -> String {
    if let Some(info) = &game.h2h_info {
        let opts = if info.margin == 1 { 2 } else { 3 };
        match opt_idx {
            0 => format!("{} W (H2H 2-0)", info.first_winner.name()),
            1 if opts == 2 => format!("{} W (H2H preokret)", info.first_loser.name()),
            1 => format!("{} W ({} vodi H2H diff)", info.first_loser.name(), info.first_winner.name()),
            2 => format!("{} W ({} vodi H2H diff)", info.first_loser.name(), info.first_loser.name()),
            _ => "?".into(),
        }
    } else {
        match opt_idx {
            0 => format!("{} W", game.home.name()),
            1 => format!("{} W", game.away.name()),
            _ => "?".into(),
        }
    }
}

fn build_game_options(game: &RemainingGame) -> Vec<GameOutcome> {
    if let Some(info) = &game.h2h_info {
        let first_winner_wins = if info.first_winner == game.home {
            GameOutcome::HomeWin
        } else {
            GameOutcome::AwayWin
        };
        if info.margin == 1 {
            vec![first_winner_wins, GameOutcome::H2HLoserWinsBig]
        } else {
            vec![first_winner_wins, GameOutcome::H2HLoserWinsSmall, GameOutcome::H2HLoserWinsBig]
        }
    } else {
        vec![GameOutcome::HomeWin, GameOutcome::AwayWin]
    }
}

// ============================================================================
// Handlers
// ============================================================================

async fn index_handler() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn games_handler() -> Json<Vec<GameInfo>> {
    let games = remaining_games();
    let base = base_standings();

    // Compute max possible RED wins (all 2 games won)
    let red_max_wins = 20 + 2; // RED currently has 20W, 2 remaining games

    let mut team_remaining: HashMap<Team, u32> = HashMap::new();
    for g in &games {
        *team_remaining.entry(g.home).or_insert(0) += 1;
        *team_remaining.entry(g.away).or_insert(0) += 1;
    }
    // RED's own remaining count not needed for relevance check, only opponents

    let result: Vec<GameInfo> = games.iter().enumerate().map(|(i, g)| {
        let is_red = g.home == Team::RED || g.away == Team::RED;

        // Check if H2H detail matters (can these 2 teams finish with same wins?)
        let h2h_dominated = g.h2h_info.is_some() && {
            let sh = base.iter().find(|s| s.team == g.home).unwrap();
            let sa = base.iter().find(|s| s.team == g.away).unwrap();
            let rh = team_remaining.get(&g.home).copied().unwrap_or(0);
            let ra = team_remaining.get(&g.away).copied().unwrap_or(0);
            sh.wins + rh < sa.wins || sa.wins + ra < sh.wins
        };

        let opt_labels: Vec<String> = if h2h_dominated {
            vec![format!("{} W", g.home.name()), format!("{} W", g.away.name())]
        } else {
            let opts = build_game_options(g);
            (0..opts.len()).map(|j| describe_option(g, j)).collect()
        };

        // A game is relevant if at least one team could potentially tie RED at any RED win count
        let home_relevant = RELEVANT_TEAMS.contains(&g.home) && {
            let s = base.iter().find(|s| s.team == g.home).unwrap();
            let rem = team_remaining.get(&g.home).copied().unwrap_or(0);
            // Could tie at RED's minimum wins (20) through maximum wins (22)
            s.wins + rem >= 20 && s.wins <= red_max_wins
        };
        let away_relevant = RELEVANT_TEAMS.contains(&g.away) && {
            let s = base.iter().find(|s| s.team == g.away).unwrap();
            let rem = team_remaining.get(&g.away).copied().unwrap_or(0);
            s.wins + rem >= 20 && s.wins <= red_max_wins
        };

        let h2h_first_game = if h2h_dominated { None } else {
            g.h2h_info.as_ref().map(|info| {
                format!("{} W +{} vs {}", info.first_winner.name(), info.margin, info.first_loser.name())
            })
        };

        GameInfo {
            index: i,
            home: g.home.name().to_string(),
            away: g.away.name().to_string(),
            gamecode: g.gamecode,
            is_red_game: is_red,
            options: opt_labels,
            relevant: is_red || home_relevant || away_relevant,
            h2h_first_game,
        }
    }).collect();

    Json(result)
}

async fn simulate_handler(Json(req): Json<SimulateRequest>) -> Json<SimulateResponse> {
    let start = std::time::Instant::now();
    let base = base_standings();
    let h2h = base_h2h();
    let games = remaining_games();
    let n_all_games = games.len();

    // Precompute H2H simplification: if two teams can't finish with same wins,
    // their H2H tiebreaker detail is irrelevant — reduce to 2 options
    let mut total_remaining: HashMap<Team, u32> = HashMap::new();
    for g in &games {
        *total_remaining.entry(g.home).or_insert(0) += 1;
        *total_remaining.entry(g.away).or_insert(0) += 1;
    }
    let h2h_simplified: Vec<bool> = games.iter().map(|g| {
        if g.h2h_info.is_none() { return false; }
        let sh = base.iter().find(|s| s.team == g.home).unwrap();
        let sa = base.iter().find(|s| s.team == g.away).unwrap();
        let rh = total_remaining.get(&g.home).copied().unwrap_or(0);
        let ra = total_remaining.get(&g.away).copied().unwrap_or(0);
        sh.wins + rh < sa.wins || sa.wins + ra < sh.wins
    }).collect();

    // Build outcome options for each game (simplified where H2H can't matter)
    let all_options: Vec<Vec<GameOutcome>> = games.iter().enumerate().map(|(i, g)| {
        if h2h_simplified[i] {
            vec![GameOutcome::HomeWin, GameOutcome::AwayWin]
        } else {
            build_game_options(g)
        }
    }).collect();

    // Separate locked vs free games
    let mut fixed_outcomes: Vec<(usize, GameOutcome)> = Vec::new();
    let mut free_indices: Vec<usize> = Vec::new();

    for (i, opts) in all_options.iter().enumerate() {
        if let Some(&choice) = req.locked.get(&i) {
            if choice < opts.len() {
                fixed_outcomes.push((i, opts[choice]));
            } else {
                fixed_outcomes.push((i, opts[0])); // fallback
            }
        } else {
            free_indices.push(i);
        }
    }

    // Determine which free games are critical vs irrelevant
    // We need to know RED's wins from locked RED games
    let red_game_indices: Vec<usize> = games.iter().enumerate()
        .filter(|(_, g)| g.home == Team::RED || g.away == Team::RED)
        .map(|(i, _)| i)
        .collect();

    // Check if all RED games are locked
    let all_red_locked = red_game_indices.iter().all(|i| req.locked.contains_key(i));

    // Compute RED's possible win range
    let red_base_wins = 20u32;
    let mut red_min_wins = red_base_wins;
    let mut red_max_wins = red_base_wins;

    for &gi in &red_game_indices {
        let g = &games[gi];
        if let Some(&choice) = req.locked.get(&gi) {
            let opts = &all_options[gi];
            let outcome = opts[choice.min(opts.len() - 1)];
            let red_wins = match outcome {
                GameOutcome::HomeWin => g.home == Team::RED,
                GameOutcome::AwayWin => g.away == Team::RED,
                GameOutcome::H2HLoserWinsSmall | GameOutcome::H2HLoserWinsBig => {
                    let info = g.h2h_info.as_ref().unwrap();
                    info.first_loser == Team::RED
                }
            };
            if red_wins {
                red_min_wins += 1;
                red_max_wins += 1;
            }
        } else {
            // RED game not locked, RED could win or lose
            red_max_wins += 1;
        }
    }

    // `can_tie` for the widest possible RED win range
    let mut team_remaining: HashMap<Team, u32> = HashMap::new();
    for &gi in &free_indices {
        let g = &games[gi];
        *team_remaining.entry(g.home).or_insert(0) += 1;
        *team_remaining.entry(g.away).or_insert(0) += 1;
    }

    let can_tie: BTreeSet<Team> = RELEVANT_TEAMS.iter()
        .filter(|&&t| {
            if t == Team::RED { return false; }
            let s = base.iter().find(|s| s.team == t).unwrap();
            let locked_wins: u32 = fixed_outcomes.iter()
                .filter(|&&(gi, outcome)| {
                    let g = &games[gi];
                    if g.home != t && g.away != t { return false; }
                    let winner = match outcome {
                        GameOutcome::HomeWin => g.home,
                        GameOutcome::AwayWin => g.away,
                        GameOutcome::H2HLoserWinsSmall | GameOutcome::H2HLoserWinsBig => {
                            g.h2h_info.as_ref().unwrap().first_loser
                        }
                    };
                    winner == t
                })
                .count() as u32;
            let total_wins = s.wins + locked_wins;
            let rem = team_remaining.get(&t).copied().unwrap_or(0);
            total_wins + rem >= red_min_wins && total_wins <= red_max_wins
        })
        .copied()
        .collect();

    // Separate free games into critical and irrelevant
    let mut critical_free: Vec<usize> = Vec::new();
    let mut irrelevant_free: Vec<usize> = Vec::new();

    for &gi in &free_indices {
        let g = &games[gi];
        let is_red = g.home == Team::RED || g.away == Team::RED;
        if is_red || can_tie.contains(&g.home) || can_tie.contains(&g.away) {
            critical_free.push(gi);
        } else {
            irrelevant_free.push(gi);
        }
    }

    // Fix irrelevant free games to home win
    for &gi in &irrelevant_free {
        fixed_outcomes.push((gi, GameOutcome::HomeWin));
    }

    // Build enumeration
    let game_options: Vec<(usize, Vec<GameOutcome>)> = critical_free.iter()
        .map(|&gi| (gi, all_options[gi].clone()))
        .collect();

    // Precompute weights for critical games
    let game_weights: Vec<Vec<f64>> = critical_free.iter()
        .map(|&gi| {
            if h2h_simplified[gi] {
                vec![0.5, 0.5]
            } else {
                outcome_weights(&games[gi])
            }
        })
        .collect();

    let radixes: Vec<usize> = game_options.iter().map(|(_, opts)| opts.len()).collect();
    let n_crit = game_options.len();

    let actual_combos: u64 = if game_options.is_empty() { 1 } else {
        game_options.iter().map(|(_, opts)| opts.len() as u64).product()
    };

    // Weighted accumulators (f64)
    struct PosAccum {
        total: f64,
        opponents: BTreeMap<Team, f64>,
        game_winner_home: Vec<f64>,
        game_winner_away: Vec<f64>,
    }

    let accum: Mutex<BTreeMap<u32, PosAccum>> = Mutex::new(BTreeMap::new());
    let eliminated: Mutex<f64> = Mutex::new(0.0);

    // Impact accumulators: per critical game, per option -> (weighted_pos_sum, weight_sum)
    let impact_pos: Mutex<Vec<Vec<f64>>> = Mutex::new(radixes.iter().map(|&r| vec![0.0; r]).collect());
    let impact_wt: Mutex<Vec<Vec<f64>>> = Mutex::new(radixes.iter().map(|&r| vec![0.0; r]).collect());
    let overall_pos: Mutex<f64> = Mutex::new(0.0);
    let overall_wt: Mutex<f64> = Mutex::new(0.0);

    let chunk_size: u64 = if actual_combos > 100_000 { 10_000 } else { 1 };
    let n_chunks = (actual_combos + chunk_size - 1) / chunk_size;

    (0..n_chunks).into_par_iter().for_each(|chunk_idx| {
        let start_c = chunk_idx * chunk_size;
        let end_c = std::cmp::min(start_c + chunk_size, actual_combos);

        let mut local_pos: BTreeMap<u32, (f64, BTreeMap<Team, f64>, Vec<f64>, Vec<f64>)> = BTreeMap::new();
        let mut local_eliminated: f64 = 0.0;
        let mut local_impact_pos: Vec<Vec<f64>> = radixes.iter().map(|&r| vec![0.0; r]).collect();
        let mut local_impact_wt: Vec<Vec<f64>> = radixes.iter().map(|&r| vec![0.0; r]).collect();
        let mut local_overall_pos: f64 = 0.0;
        let mut local_overall_wt: f64 = 0.0;

        for combo_num in start_c..end_c {
            let mut choices = vec![0usize; n_crit];
            let mut remaining = combo_num;
            for i in 0..n_crit {
                let radix = radixes[i] as u64;
                choices[i] = (remaining % radix) as usize;
                remaining /= radix;
            }

            // Compute scenario weight from critical game choices
            let mut weight: f64 = 1.0;
            for i in 0..n_crit {
                weight *= game_weights[i][choices[i]];
            }

            let mut outcomes = fixed_outcomes.clone();
            for i in 0..n_crit {
                let (gi, ref opts) = game_options[i];
                outcomes.push((gi, opts[choices[i]]));
            }

            let result = run_scenario(&outcomes, &games, &base, &h2h);

            // Accumulate expected position for impact analysis (ALL scenarios)
            let effective_pos: f64 = if result.has_unresolved_red || result.red_min != result.red_max {
                (result.red_min as f64 + result.red_max as f64) / 2.0
            } else {
                result.red_min as f64
            };
            local_overall_pos += weight * effective_pos;
            local_overall_wt += weight;
            for i in 0..n_crit {
                local_impact_pos[i][choices[i]] += weight * effective_pos;
                local_impact_wt[i][choices[i]] += weight;
            }

            if result.has_unresolved_red || result.red_min != result.red_max {
                local_eliminated += weight;
                continue;
            }

            let red_pos = result.red_min;
            if red_pos > 10 {
                local_eliminated += weight;
                continue;
            }

            let entry = local_pos.entry(red_pos).or_insert_with(|| {
                (0.0, BTreeMap::new(), vec![0.0f64; n_all_games], vec![0.0f64; n_all_games])
            });
            entry.0 += weight;

            if let Some(opp_pos) = opponent_position(red_pos) {
                let opp_team = result.standings[(opp_pos - 1) as usize];
                *entry.1.entry(opp_team).or_insert(0.0) += weight;
            }

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
                    entry.2[gi] += weight;
                } else {
                    entry.3[gi] += weight;
                }
            }
        }

        // Merge
        {
            let mut ga = accum.lock().unwrap();
            for (pos, (count, opps, home_w, away_w)) in &local_pos {
                let entry = ga.entry(*pos).or_insert_with(|| PosAccum {
                    total: 0.0,
                    opponents: BTreeMap::new(),
                    game_winner_home: vec![0.0f64; n_all_games],
                    game_winner_away: vec![0.0f64; n_all_games],
                });
                entry.total += count;
                for (&t, &c) in opps {
                    *entry.opponents.entry(t).or_insert(0.0) += c;
                }
                for i in 0..n_all_games {
                    entry.game_winner_home[i] += home_w[i];
                    entry.game_winner_away[i] += away_w[i];
                }
            }
        }
        {
            *eliminated.lock().unwrap() += local_eliminated;
        }
        {
            *overall_pos.lock().unwrap() += local_overall_pos;
            *overall_wt.lock().unwrap() += local_overall_wt;
            let mut ip = impact_pos.lock().unwrap();
            let mut iw = impact_wt.lock().unwrap();
            for i in 0..n_crit {
                for j in 0..radixes[i] {
                    ip[i][j] += local_impact_pos[i][j];
                    iw[i][j] += local_impact_wt[i][j];
                }
            }
        }
    });

    // Build response
    let data = accum.lock().unwrap();
    let elim = *eliminated.lock().unwrap();
    let total_weight: f64 = data.values().map(|pd| pd.total).sum::<f64>() + elim;

    let mut positions = Vec::new();
    for (&pos, pd) in data.iter() {
        let total = pd.total;

        // Opponents
        let mut opp_vec: Vec<(Team, f64)> = pd.opponents.iter()
            .map(|(&t, &c)| (t, c)).collect();
        opp_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let opponents: Vec<OpponentInfo> = opp_vec.iter()
            .filter(|(_, c)| (*c / total * 100.0) >= 0.5)
            .map(|(t, c)| OpponentInfo {
                team: t.name().to_string(),
                pct: *c / total * 100.0,
            })
            .collect();

        // Game results (only critical games — skip irrelevant pruned games)
        let mut game_results = Vec::new();
        for gi in 0..n_all_games {
            let hw = pd.game_winner_home[gi];
            let aw = pd.game_winner_away[gi];
            if hw + aw < 1e-12 { continue; }
            let g = &games[gi];
            let is_red_game = g.home == Team::RED || g.away == Team::RED;
            // Skip games where neither team can tie RED (these were pruned/fixed), but keep RED's own
            if !is_red_game && !can_tie.contains(&g.home) && !can_tie.contains(&g.away) { continue; }

            let total_g = hw + aw;
            let home_pct = hw / total_g * 100.0;

            let required_winner = if home_pct >= 99.9 {
                Some(g.home.name().to_string())
            } else if home_pct <= 0.1 {
                Some(g.away.name().to_string())
            } else {
                None
            };

            game_results.push(GameResultInfo {
                index: gi,
                home: g.home.name().to_string(),
                away: g.away.name().to_string(),
                home_pct,
                required_winner,
            });
        }

        // Sort: required first
        game_results.sort_by(|a, b| {
            let a_req = a.required_winner.is_some();
            let b_req = b.required_winner.is_some();
            b_req.cmp(&a_req)
                .then(b.home_pct.partial_cmp(&a.home_pct).unwrap_or(std::cmp::Ordering::Equal))
        });

        positions.push(PositionResult {
            position: pos,
            round_name: playoff_round_name(pos).to_string(),
            count: actual_combos,
            pct: total / total_weight * 100.0,
            opponents,
            game_results,
        });
    }

    // Build game impacts
    let ip = impact_pos.lock().unwrap();
    let iw = impact_wt.lock().unwrap();
    let ow = *overall_wt.lock().unwrap();
    let expected_position = if ow > 0.0 { *overall_pos.lock().unwrap() / ow } else { 0.0 };

    let mut game_impacts = Vec::new();
    for (ci, &(gi, ref _opts)) in game_options.iter().enumerate() {
        let g = &games[gi];
        let opts_count = radixes[ci];
        let mut options = Vec::new();
        for oi in 0..opts_count {
            let exp = if iw[ci][oi] > 0.0 { ip[ci][oi] / iw[ci][oi] } else { 0.0 };
            let label = if h2h_simplified[gi] {
                match oi { 0 => format!("{} W", g.home.name()), _ => format!("{} W", g.away.name()) }
            } else {
                describe_option(g, oi)
            };
            options.push(OptionImpact {
                label,
                expected_pos: exp,
                delta: exp - expected_position,
            });
        }
        game_impacts.push(GameImpact {
            index: gi,
            home: g.home.name().to_string(),
            away: g.away.name().to_string(),
            options,
        });
    }

    let elapsed = start.elapsed();

    Json(SimulateResponse {
        total_scenarios: actual_combos,
        elapsed_ms: elapsed.as_millis() as u64,
        positions,
        eliminated_count: actual_combos,
        eliminated_pct: elim / total_weight * 100.0,
        expected_position,
        game_impacts,
    })
}

// ============================================================================
// Main
// ============================================================================

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/games", get(games_handler))
        .route("/api/simulate", post(simulate_handler));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Euroleague Scenarios web server running on http://localhost:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
