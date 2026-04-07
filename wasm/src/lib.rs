use euroleague_scenarios::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use wasm_bindgen::prelude::*;

// ============================================================================
// API types (mirror web.rs but for WASM)
// ============================================================================

#[derive(Deserialize)]
struct SimulateRequest {
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
    relevant: bool,
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
struct SimulateResponse {
    total_scenarios: u64,
    elapsed_ms: u64,
    positions: Vec<PositionResult>,
    eliminated_count: u64,
    eliminated_pct: f64,
}

// ============================================================================
// Helpers (same as web.rs)
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
// WASM exports
// ============================================================================

#[wasm_bindgen]
pub fn get_games() -> JsValue {
    let games = remaining_games();
    let base = base_standings();

    let red_max_wins = 20 + 2;

    let mut team_remaining: HashMap<Team, u32> = HashMap::new();
    for g in &games {
        *team_remaining.entry(g.home).or_insert(0) += 1;
        *team_remaining.entry(g.away).or_insert(0) += 1;
    }

    let result: Vec<GameInfo> = games.iter().enumerate().map(|(i, g)| {
        let is_red = g.home == Team::RED || g.away == Team::RED;
        let opts = build_game_options(g);

        let home_relevant = RELEVANT_TEAMS.contains(&g.home) && {
            let s = base.iter().find(|s| s.team == g.home).unwrap();
            let rem = team_remaining.get(&g.home).copied().unwrap_or(0);
            s.wins + rem >= 20 && s.wins <= red_max_wins
        };
        let away_relevant = RELEVANT_TEAMS.contains(&g.away) && {
            let s = base.iter().find(|s| s.team == g.away).unwrap();
            let rem = team_remaining.get(&g.away).copied().unwrap_or(0);
            s.wins + rem >= 20 && s.wins <= red_max_wins
        };

        let h2h_first_game = g.h2h_info.as_ref().map(|info| {
            format!("{} W +{} vs {}", info.first_winner.name(), info.margin, info.first_loser.name())
        });

        GameInfo {
            index: i,
            home: g.home.name().to_string(),
            away: g.away.name().to_string(),
            gamecode: g.gamecode,
            is_red_game: is_red,
            options: (0..opts.len()).map(|j| describe_option(g, j)).collect(),
            relevant: is_red || home_relevant || away_relevant,
            h2h_first_game,
        }
    }).collect();

    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn simulate(locked_json: &str) -> JsValue {
    let req: SimulateRequest = serde_json::from_str(locked_json).unwrap_or(SimulateRequest {
        locked: HashMap::new(),
    });

    let start = js_sys_now();
    let base = base_standings();
    let h2h = base_h2h();
    let games = remaining_games();
    let n_all_games = games.len();

    let all_options: Vec<Vec<GameOutcome>> = games.iter().map(|g| build_game_options(g)).collect();

    let mut fixed_outcomes: Vec<(usize, GameOutcome)> = Vec::new();
    let mut free_indices: Vec<usize> = Vec::new();

    for (i, opts) in all_options.iter().enumerate() {
        if let Some(&choice) = req.locked.get(&i) {
            if choice < opts.len() {
                fixed_outcomes.push((i, opts[choice]));
            } else {
                fixed_outcomes.push((i, opts[0]));
            }
        } else {
            free_indices.push(i);
        }
    }

    let red_game_indices: Vec<usize> = games.iter().enumerate()
        .filter(|(_, g)| g.home == Team::RED || g.away == Team::RED)
        .map(|(i, _)| i)
        .collect();

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
            red_max_wins += 1;
        }
    }

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

    for &gi in &irrelevant_free {
        fixed_outcomes.push((gi, GameOutcome::HomeWin));
    }

    let game_options: Vec<(usize, Vec<GameOutcome>)> = critical_free.iter()
        .map(|&gi| (gi, all_options[gi].clone()))
        .collect();

    let game_weights: Vec<Vec<f64>> = critical_free.iter()
        .map(|&gi| outcome_weights(&games[gi]))
        .collect();

    let radixes: Vec<usize> = game_options.iter().map(|(_, opts)| opts.len()).collect();
    let n_crit = game_options.len();

    let actual_combos: u64 = if game_options.is_empty() { 1 } else {
        game_options.iter().map(|(_, opts)| opts.len() as u64).product()
    };

    // Single-threaded enumeration (no rayon in WASM)
    let mut pos_data: BTreeMap<u32, (f64, BTreeMap<Team, f64>, Vec<f64>, Vec<f64>)> = BTreeMap::new();
    let mut eliminated: f64 = 0.0;

    for combo_num in 0..actual_combos {
        let mut choices = vec![0usize; n_crit];
        let mut remaining = combo_num;
        for i in 0..n_crit {
            let radix = radixes[i] as u64;
            choices[i] = (remaining % radix) as usize;
            remaining /= radix;
        }

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

        if result.has_unresolved_red || result.red_min != result.red_max {
            eliminated += weight;
            continue;
        }

        let red_pos = result.red_min;
        if red_pos > 10 {
            eliminated += weight;
            continue;
        }

        let entry = pos_data.entry(red_pos).or_insert_with(|| {
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

    // Build response
    let total_weight: f64 = pos_data.values().map(|pd| pd.0).sum::<f64>() + eliminated;

    let mut positions = Vec::new();
    for (&pos, pd) in pos_data.iter() {
        let total = pd.0;

        let mut opp_vec: Vec<(Team, f64)> = pd.1.iter().map(|(&t, &c)| (t, c)).collect();
        opp_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let opponents: Vec<OpponentInfo> = opp_vec.iter()
            .filter(|(_, c)| (*c / total * 100.0) >= 0.5)
            .map(|(t, c)| OpponentInfo {
                team: t.name().to_string(),
                pct: *c / total * 100.0,
            })
            .collect();

        let mut game_results = Vec::new();
        for gi in 0..n_all_games {
            let hw = pd.2[gi];
            let aw = pd.3[gi];
            if hw + aw < 1e-12 { continue; }
            let g = &games[gi];
            let is_red_game = g.home == Team::RED || g.away == Team::RED;
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

    let elapsed_ms = (js_sys_now() - start) as u64;

    let response = SimulateResponse {
        total_scenarios: actual_combos,
        elapsed_ms,
        positions,
        eliminated_count: actual_combos,
        eliminated_pct: eliminated / total_weight * 100.0,
    };

    serde_wasm_bindgen::to_value(&response).unwrap()
}

// JS performance.now() via wasm-bindgen
fn js_sys_now() -> f64 {
    js_sys::Date::now()
}
