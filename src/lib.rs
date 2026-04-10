use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};

// ============================================================================
// DATA: Teams
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[repr(u8)]
pub enum Team {
    OLY, ULK, MAD, PAM, ZAL, HTA, PAN, BAR, MCO, RED, DUB, TEL, MIL,
    // Non-relevant teams (needed for game encoding but not standings analysis)
    MUN, PAR, PRS, VIR, BAS, IST, ASV,
}

impl Team {
    pub fn code(self) -> &'static str {
        match self {
            Team::OLY => "OLY", Team::ULK => "ULK", Team::MAD => "MAD",
            Team::PAM => "PAM", Team::ZAL => "ZAL", Team::HTA => "HTA",
            Team::PAN => "PAN", Team::BAR => "BAR", Team::MCO => "MCO",
            Team::RED => "RED", Team::DUB => "DUB", Team::TEL => "TEL",
            Team::MIL => "MIL", Team::MUN => "MUN", Team::PAR => "PAR",
            Team::PRS => "PRS", Team::VIR => "VIR", Team::BAS => "BAS",
            Team::IST => "IST", Team::ASV => "ASV",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Team::OLY => "Olympiacos",
            Team::ULK => "Fenerbahče",
            Team::MAD => "Real Madrid",
            Team::PAM => "Valencia Basket",
            Team::ZAL => "Žalgiris",
            Team::HTA => "Hapoel Tel Aviv",
            Team::PAN => "Panathinaikos",
            Team::BAR => "Barcelona",
            Team::MCO => "Monaco",
            Team::RED => "Crvena zvezda",
            Team::DUB => "Dubai Basketball",
            Team::TEL => "Maccabi Tel Aviv",
            Team::MIL => "EA7 Milano",
            Team::MUN => "Bayern München",
            Team::PAR => "Partizan",
            Team::PRS => "Paris Basketball",
            Team::VIR => "Virtus Bologna",
            Team::BAS => "Baskonia",
            Team::IST => "Anadolu Efes",
            Team::ASV => "ASVEL Villeurbanne",
        }
    }
}

pub const ALL_TEAMS: [Team; 20] = [
    Team::OLY, Team::ULK, Team::MAD, Team::PAM, Team::ZAL, Team::HTA,
    Team::PAN, Team::BAR, Team::MCO, Team::RED, Team::DUB, Team::TEL,
    Team::MIL, Team::MUN, Team::PAR, Team::PRS, Team::VIR, Team::BAS,
    Team::IST, Team::ASV,
];

pub const RELEVANT_TEAMS: [Team; 13] = [
    Team::OLY, Team::ULK, Team::MAD, Team::PAM, Team::ZAL, Team::HTA,
    Team::PAN, Team::BAR, Team::MCO, Team::RED, Team::DUB, Team::TEL,
    Team::MIL,
];

// ============================================================================
// DATA: Current standings (after R37 complete; TEL 36gp, HTA 36gp, all others 37gp)
// ============================================================================

#[derive(Debug, Clone)]
pub struct TeamStats {
    pub team: Team,
    pub gp: u32,
    pub wins: u32,
    pub losses: u32,
    pub pf_reg: i32,
    pub pa_reg: i32,
    pub diff_reg: i32,
}

pub fn base_standings() -> Vec<TeamStats> {
    vec![
        TeamStats { team: Team::OLY, gp: 37, wins: 25, losses: 12, pf_reg: 3321, pa_reg: 3068, diff_reg: 253 },
        TeamStats { team: Team::ULK, gp: 37, wins: 23, losses: 14, pf_reg: 3033, pa_reg: 2985, diff_reg: 48 },
        TeamStats { team: Team::MAD, gp: 37, wins: 23, losses: 14, pf_reg: 3239, pa_reg: 3074, diff_reg: 165 },
        TeamStats { team: Team::PAM, gp: 37, wins: 24, losses: 13, pf_reg: 3323, pa_reg: 3158, diff_reg: 165 },
        TeamStats { team: Team::ZAL, gp: 37, wins: 22, losses: 15, pf_reg: 3219, pa_reg: 3046, diff_reg: 173 },
        TeamStats { team: Team::HTA, gp: 36, wins: 22, losses: 14, pf_reg: 3145, pa_reg: 3018, diff_reg: 127 },
        TeamStats { team: Team::PAN, gp: 37, wins: 21, losses: 16, pf_reg: 3217, pa_reg: 3166, diff_reg: 51 },
        TeamStats { team: Team::BAR, gp: 37, wins: 20, losses: 17, pf_reg: 3072, pa_reg: 3078, diff_reg: -6 },
        TeamStats { team: Team::MCO, gp: 37, wins: 21, losses: 16, pf_reg: 3312, pa_reg: 3197, diff_reg: 115 },
        TeamStats { team: Team::RED, gp: 37, wins: 21, losses: 16, pf_reg: 3205, pa_reg: 3142, diff_reg: 63 },
        TeamStats { team: Team::DUB, gp: 37, wins: 19, losses: 18, pf_reg: 3239, pa_reg: 3230, diff_reg: 9 },
        TeamStats { team: Team::TEL, gp: 36, wins: 18, losses: 18, pf_reg: 3213, pa_reg: 3298, diff_reg: -85 },
        TeamStats { team: Team::MIL, gp: 37, wins: 17, losses: 20, pf_reg: 3170, pa_reg: 3209, diff_reg: -39 },
        TeamStats { team: Team::MUN, gp: 37, wins: 17, losses: 20, pf_reg: 2994, pa_reg: 3073, diff_reg: -79 },
        TeamStats { team: Team::PAR, gp: 37, wins: 15, losses: 22, pf_reg: 2961, pa_reg: 3163, diff_reg: -202 },
        TeamStats { team: Team::PRS, gp: 37, wins: 15, losses: 22, pf_reg: 3343, pa_reg: 3371, diff_reg: -28 },
        TeamStats { team: Team::VIR, gp: 37, wins: 13, losses: 24, pf_reg: 3021, pa_reg: 3200, diff_reg: -179 },
        TeamStats { team: Team::BAS, gp: 37, wins: 13, losses: 24, pf_reg: 3242, pa_reg: 3392, diff_reg: -150 },
        TeamStats { team: Team::IST, gp: 37, wins: 12, losses: 25, pf_reg: 2929, pa_reg: 3054, diff_reg: -125 },
        TeamStats { team: Team::ASV, gp: 37, wins: 8,  losses: 29, pf_reg: 2913, pa_reg: 3189, diff_reg: -276 },
    ]
}

// ============================================================================
// DATA: H2H records (current state from completed games)
// ============================================================================

#[derive(Debug, Clone)]
pub struct H2HRecord {
    pub team_a: Team,
    pub team_b: Team,
    pub games_played: u32,
    pub wins_a: u32,
    pub wins_b: u32,
    pub diff_a: i32,
    pub diff_b: i32,
    pub met_twice: bool,
}

pub fn h2h_key(a: Team, b: Team) -> (Team, Team) {
    if a <= b { (a, b) } else { (b, a) }
}

pub fn base_h2h() -> HashMap<(Team, Team), H2HRecord> {
    let data: Vec<(Team, Team, u32, u32, u32, i32, i32, bool)> = vec![
        (Team::BAR, Team::DUB, 2, 1, 1, -3, 3, true),
        (Team::BAR, Team::HTA, 2, 0, 2, -21, 21, true),
        (Team::BAR, Team::MAD, 2, 0, 2, -28, 28, true),
        (Team::BAR, Team::MCO, 2, 0, 2, -23, 23, true),
        (Team::BAR, Team::MIL, 2, 1, 1, -1, 1, true),
        (Team::BAR, Team::OLY, 2, 1, 1, 1, -1, true),
        (Team::BAR, Team::PAM, 2, 2, 0, 10, -10, true),
        (Team::BAR, Team::PAN, 2, 1, 1, -7, 7, true),
        (Team::BAR, Team::RED, 2, 2, 0, 10, -10, true),
        (Team::BAR, Team::TEL, 2, 2, 0, 31, -31, true),
        (Team::BAR, Team::ULK, 2, 0, 2, -5, 5, true),
        (Team::BAR, Team::ZAL, 2, 0, 2, -27, 27, true),
        (Team::DUB, Team::HTA, 2, 0, 2, -37, 37, true),
        (Team::DUB, Team::MAD, 2, 1, 1, -6, 6, true),
        (Team::DUB, Team::MCO, 2, 1, 1, -12, 12, true),
        (Team::DUB, Team::MIL, 2, 2, 0, 25, -25, true),
        (Team::DUB, Team::OLY, 2, 1, 1, -19, 19, true),
        (Team::DUB, Team::PAM, 1, 0, 1, -2, 2, false),
        (Team::DUB, Team::PAN, 2, 0, 2, -24, 24, true),
        (Team::DUB, Team::RED, 2, 1, 1, 13, -13, true),
        (Team::DUB, Team::TEL, 2, 0, 2, -2, 2, true),
        (Team::DUB, Team::ULK, 2, 2, 0, 35, -35, true),
        (Team::DUB, Team::ZAL, 2, 2, 0, 18, -18, true),
        (Team::HTA, Team::MAD, 2, 0, 2, -10, 10, true),
        (Team::HTA, Team::MCO, 1, 1, 0, 8, -8, false),
        (Team::HTA, Team::MIL, 2, 2, 0, 28, -28, true),
        (Team::HTA, Team::OLY, 2, 0, 2, -8, 8, true),
        (Team::HTA, Team::PAM, 2, 1, 1, 7, -7, true),
        (Team::HTA, Team::PAN, 2, 1, 1, -7, 7, true),
        (Team::HTA, Team::RED, 2, 1, 1, -6, 6, true),
        (Team::HTA, Team::TEL, 1, 0, 1, -13, 13, false),
        (Team::HTA, Team::ULK, 2, 1, 1, 9, -9, true),
        (Team::HTA, Team::ZAL, 2, 0, 2, -24, 24, true),
        (Team::MAD, Team::MCO, 2, 1, 1, 7, -7, true),
        (Team::MAD, Team::MIL, 2, 1, 1, 22, -22, true),
        (Team::MAD, Team::OLY, 2, 1, 1, -2, 2, true),
        (Team::MAD, Team::PAM, 2, 1, 1, 4, -4, true),
        (Team::MAD, Team::PAN, 2, 0, 2, -11, 11, true),
        (Team::MAD, Team::RED, 1, 0, 1, -15, 15, false),
        (Team::MAD, Team::TEL, 2, 1, 1, 11, -11, true),
        (Team::MAD, Team::ULK, 2, 2, 0, 31, -31, true),
        (Team::MAD, Team::ZAL, 2, 1, 1, -1, 1, true),
        (Team::MCO, Team::MIL, 2, 2, 0, 8, -8, true),
        (Team::MCO, Team::OLY, 2, 2, 0, 6, -6, true),
        (Team::MCO, Team::PAM, 2, 2, 0, 15, -15, true),
        (Team::MCO, Team::PAN, 2, 1, 1, -2, 2, true),
        (Team::MCO, Team::RED, 2, 0, 2, -12, 12, true),
        (Team::MCO, Team::TEL, 2, 1, 1, -5, 5, true),
        (Team::MCO, Team::ULK, 2, 0, 2, -24, 24, true),
        (Team::MCO, Team::ZAL, 2, 0, 2, -22, 22, true),
        (Team::MIL, Team::OLY, 1, 1, 0, 1, -1, false),
        (Team::MIL, Team::PAM, 2, 0, 2, -9, 9, true),
        (Team::MIL, Team::PAN, 2, 2, 0, 20, -20, true),
        (Team::MIL, Team::RED, 2, 1, 1, 2, -2, true),
        (Team::MIL, Team::TEL, 2, 2, 0, 23, -23, true),
        (Team::MIL, Team::ULK, 2, 0, 2, -19, 19, true),
        (Team::MIL, Team::ZAL, 2, 1, 1, 7, -7, true),
        (Team::OLY, Team::PAM, 2, 0, 2, -8, 8, true),
        (Team::OLY, Team::PAN, 2, 2, 0, 11, -11, true),
        (Team::OLY, Team::RED, 2, 1, 1, -5, 5, true),
        (Team::OLY, Team::TEL, 2, 2, 0, 8, -8, true),
        (Team::OLY, Team::ULK, 2, 1, 1, 9, -9, true),
        (Team::OLY, Team::ZAL, 2, 1, 1, 17, -17, true),
        (Team::PAM, Team::PAN, 2, 2, 0, 28, -28, true),
        (Team::PAM, Team::RED, 2, 2, 0, 20, -20, true),
        (Team::PAM, Team::TEL, 2, 1, 1, 8, -8, true),
        (Team::PAM, Team::ULK, 2, 1, 1, 12, -12, true),
        (Team::PAM, Team::ZAL, 2, 1, 1, -5, 5, true),
        (Team::PAN, Team::RED, 2, 1, 1, -10, 10, true),
        (Team::PAN, Team::TEL, 2, 1, 1, 10, -10, true),
        (Team::PAN, Team::ULK, 2, 1, 1, 2, -2, true),
        (Team::PAN, Team::ZAL, 2, 2, 0, 11, -11, true),
        (Team::RED, Team::TEL, 2, 1, 1, 6, -6, true),
        (Team::RED, Team::ULK, 2, 2, 0, 11, -11, true),
        (Team::RED, Team::ZAL, 2, 1, 1, -23, 23, true),
        (Team::TEL, Team::ULK, 2, 1, 1, -4, 4, true),
        (Team::TEL, Team::ZAL, 2, 2, 0, 21, -21, true),
        (Team::ULK, Team::ZAL, 2, 0, 2, -13, 13, true),
    ];

    let mut map = HashMap::new();
    for (a, b, gp, wa, wb, da, db, met) in data {
        let key = h2h_key(a, b);
        let (wa_n, wb_n, da_n, db_n) = if key.0 == a {
            (wa, wb, da, db)
        } else {
            (wb, wa, db, da)
        };
        map.insert(key, H2HRecord {
            team_a: key.0, team_b: key.1, games_played: gp,
            wins_a: wa_n, wins_b: wb_n, diff_a: da_n, diff_b: db_n, met_twice: met,
        });
    }
    map
}

// ============================================================================
// DATA: Remaining games
// ============================================================================

#[derive(Debug, Clone)]
pub struct H2HIncompleteInfo {
    pub first_winner: Team,
    pub first_loser: Team,
    pub margin: i32,
}

#[derive(Debug, Clone)]
pub struct RemainingGame {
    pub home: Team,
    pub away: Team,
    pub gamecode: u32,
    pub h2h_info: Option<H2HIncompleteInfo>,
}

pub fn remaining_games() -> Vec<RemainingGame> {
    vec![
        // Round 38
        RemainingGame { home: Team::ASV, away: Team::ULK, gamecode: 371, h2h_info: None },
        RemainingGame { home: Team::TEL, away: Team::VIR, gamecode: 372, h2h_info: None },
        RemainingGame { home: Team::MAD, away: Team::RED, gamecode: 374,
            h2h_info: Some(H2HIncompleteInfo { first_winner: Team::RED, first_loser: Team::MAD, margin: 15 }) },
        RemainingGame { home: Team::OLY, away: Team::MIL, gamecode: 375,
            h2h_info: Some(H2HIncompleteInfo { first_winner: Team::MIL, first_loser: Team::OLY, margin: 1 }) },
        RemainingGame { home: Team::PAN, away: Team::IST, gamecode: 376, h2h_info: None },
        RemainingGame { home: Team::MCO, away: Team::HTA, gamecode: 377,
            h2h_info: Some(H2HIncompleteInfo { first_winner: Team::HTA, first_loser: Team::MCO, margin: 8 }) },
        RemainingGame { home: Team::DUB, away: Team::PAM, gamecode: 378,
            h2h_info: Some(H2HIncompleteInfo { first_winner: Team::PAM, first_loser: Team::DUB, margin: 2 }) },
        RemainingGame { home: Team::BAR, away: Team::MUN, gamecode: 379, h2h_info: None },
        RemainingGame { home: Team::ZAL, away: Team::PRS, gamecode: 380, h2h_info: None },
        // Postponed game
        RemainingGame { home: Team::TEL, away: Team::HTA, gamecode: 293,
            h2h_info: Some(H2HIncompleteInfo { first_winner: Team::TEL, first_loser: Team::HTA, margin: 13 }) },
    ]
}

// ============================================================================
// MARGIN-BASED OUTCOME WEIGHTS
// ============================================================================

/// Returns (p_small, p_big) for a given first-game margin M.
/// p_small = P(winning margin < M) — loser wins back but first_winner still leads H2H aggregate
/// p_big   = P(winning margin > M) — loser wins back and overtakes H2H aggregate
/// Exact ties on margin are split 50/50 between small and big.
/// Based on 342 Euroleague 2025-26 games (R1-R36).
pub fn margin_split(margin: i32) -> (f64, f64) {
    // Cumulative count: CUM[m] = number of winning margins <= m (out of 342 games)
    const CUM: [u32; 42] = [
        0,   // 0
        22,  // 1
        39,  // 2
        55,  // 3
        75,  // 4
        97,  // 5
        125, // 6
        150, // 7
        164, // 8
        181, // 9
        205, // 10
        221, // 11
        236, // 12
        251, // 13
        263, // 14
        274, // 15
        282, // 16
        292, // 17
        302, // 18
        307, // 19
        309, // 20
        314, // 21
        319, // 22
        322, // 23
        324, // 24
        328, // 25
        331, // 26
        331, // 27
        332, // 28
        334, // 29
        334, // 30
        335, // 31
        338, // 32
        338, // 33
        338, // 34
        338, // 35
        339, // 36
        340, // 37
        341, // 38
        341, // 39
        341, // 40
        342, // 41
    ];
    const TOTAL: f64 = 342.0;

    let m = margin.max(1) as usize;
    let lt = if m <= 41 { CUM[m - 1] } else { 342u32 };
    let cum_m = if m <= 41 { CUM[m] } else { 342u32 };
    let eq = cum_m - lt;

    let p_small = (lt as f64 + eq as f64 * 0.5) / TOTAL;
    let p_big = 1.0 - p_small;
    (p_small, p_big)
}

/// Weights for each outcome option of a game, aligned with build_game_options().
/// Each game's weights sum to 1.0.
/// Non-H2H:  [0.5 home, 0.5 away]
/// H2H M=1:  [0.5 first_winner, 0.5 loser_big]
/// H2H M>1:  [0.5 first_winner, 0.5*p_small loser_small, 0.5*p_big loser_big]
pub fn outcome_weights(game: &RemainingGame) -> Vec<f64> {
    if let Some(info) = &game.h2h_info {
        if info.margin == 1 {
            vec![0.5, 0.5]
        } else {
            let (p_small, p_big) = margin_split(info.margin);
            vec![0.5, 0.5 * p_small, 0.5 * p_big]
        }
    } else {
        vec![0.5, 0.5]
    }
}

// ============================================================================
// ENUMERATION
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize)]
pub enum GameOutcome {
    HomeWin,
    AwayWin,
    H2HLoserWinsSmall,
    H2HLoserWinsBig,
}

#[derive(Debug, Clone)]
pub struct H2HState {
    pub records: HashMap<(Team, Team), H2HRecord>,
}

impl H2HState {
    pub fn new(base: &HashMap<(Team, Team), H2HRecord>) -> Self {
        Self { records: base.clone() }
    }

    pub fn get_wins(&self, team: Team, opponent: Team) -> u32 {
        let key = h2h_key(team, opponent);
        match self.records.get(&key) {
            Some(rec) => {
                if key.0 == team { rec.wins_a } else { rec.wins_b }
            }
            None => 0,
        }
    }

    pub fn get_diff(&self, team: Team, opponent: Team) -> i32 {
        let key = h2h_key(team, opponent);
        match self.records.get(&key) {
            Some(rec) => {
                if key.0 == team { rec.diff_a } else { rec.diff_b }
            }
            None => 0,
        }
    }

    pub fn is_complete(&self, a: Team, b: Team) -> bool {
        let key = h2h_key(a, b);
        match self.records.get(&key) {
            Some(rec) => rec.met_twice,
            None => false,
        }
    }

    pub fn apply_game(&mut self, game: &RemainingGame, outcome: GameOutcome) {
        let key = h2h_key(game.home, game.away);
        let entry = self.records.entry(key).or_insert_with(|| H2HRecord {
            team_a: key.0, team_b: key.1, games_played: 0,
            wins_a: 0, wins_b: 0, diff_a: 0, diff_b: 0, met_twice: false,
        });

        entry.games_played += 1;
        if entry.games_played >= 2 {
            entry.met_twice = true;
        }

        if let Some(info) = &game.h2h_info {
            match outcome {
                GameOutcome::HomeWin | GameOutcome::AwayWin => {
                    // first_winner wins again → 2-0
                    let winner = info.first_winner;
                    if key.0 == winner {
                        entry.wins_a += 1;
                        entry.diff_a += 8;
                        entry.diff_b -= 8;
                    } else {
                        entry.wins_b += 1;
                        entry.diff_b += 8;
                        entry.diff_a -= 8;
                    }
                }
                GameOutcome::H2HLoserWinsSmall => {
                    let winner = info.first_loser;
                    // Loser wins by (margin - 1), so aggregate diff still favors first_winner
                    let win_margin = if info.margin <= 1 { 1 } else { info.margin - 1 };
                    if key.0 == winner {
                        entry.wins_a += 1;
                        entry.diff_a += win_margin;
                        entry.diff_b -= win_margin;
                    } else {
                        entry.wins_b += 1;
                        entry.diff_b += win_margin;
                        entry.diff_a -= win_margin;
                    }
                }
                GameOutcome::H2HLoserWinsBig => {
                    let winner = info.first_loser;
                    let win_margin = info.margin + 1;
                    if key.0 == winner {
                        entry.wins_a += 1;
                        entry.diff_a += win_margin;
                        entry.diff_b -= win_margin;
                    } else {
                        entry.wins_b += 1;
                        entry.diff_b += win_margin;
                        entry.diff_a -= win_margin;
                    }
                }
            }
        } else {
            // Non-H2H game
            let winner = if matches!(outcome, GameOutcome::HomeWin) {
                game.home
            } else {
                game.away
            };
            if key.0 == winner {
                entry.wins_a += 1;
                entry.diff_a += 8;
                entry.diff_b -= 8;
            } else {
                entry.wins_b += 1;
                entry.diff_b += 8;
                entry.diff_a -= 8;
            }
        }
    }
}

// ============================================================================
// TIE-BREAK ENGINE
// ============================================================================

#[derive(Debug, Clone)]
pub enum TieSlot {
    Resolved(Team),
    Tied(Vec<Team>, String),
}

impl TieSlot {
    fn teams(&self) -> Vec<Team> {
        match self {
            TieSlot::Resolved(t) => vec![*t],
            TieSlot::Tied(ts, _) => ts.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TieResult {
    Resolved(Vec<Team>),
    Unresolved { ordering: Vec<TieSlot> },
}

pub fn resolve_tie_group(
    teams: &[Team],
    standings: &HashMap<Team, TeamStats>,
    h2h: &H2HState,
) -> TieResult {
    if teams.len() <= 1 {
        return TieResult::Resolved(teams.to_vec());
    }
    if teams.len() == 2 {
        return resolve_two_teams(teams[0], teams[1], standings, h2h);
    }
    resolve_multi_teams(teams, standings, h2h, 0)
}

fn resolve_two_teams(
    a: Team, b: Team,
    standings: &HashMap<Team, TeamStats>,
    h2h: &H2HState,
) -> TieResult {
    if h2h.is_complete(a, b) {
        let wa = h2h.get_wins(a, b);
        let wb = h2h.get_wins(b, a);
        if wa > wb { return TieResult::Resolved(vec![a, b]); }
        if wb > wa { return TieResult::Resolved(vec![b, a]); }
        // 1-1 in wins → check H2H diff
        let da = h2h.get_diff(a, b);
        let db = h2h.get_diff(b, a);
        if da > db { return TieResult::Resolved(vec![a, b]); }
        if db > da { return TieResult::Resolved(vec![b, a]); }
        // H2H diff also tied (= 0) → flag
        let sa = standings.get(&a).unwrap();
        let sb = standings.get(&b).unwrap();
        return TieResult::Unresolved {
            ordering: vec![TieSlot::Tied(
                vec![a, b],
                format!("H2H 1-1 sa izjednacenom razlikom koseva. SLEDECI KRITERIJUM: ukupan koš-razlika ({}: {}, {}: {})",
                    a.code(), sa.diff_reg, b.code(), sb.diff_reg),
            )],
        };
    }

    // Haven't met twice → 19.5.1: global criteria
    let sa = standings.get(&a).unwrap();
    let sb = standings.get(&b).unwrap();

    if sa.diff_reg != sb.diff_reg {
        return if sa.diff_reg > sb.diff_reg {
            TieResult::Resolved(vec![a, b])
        } else {
            TieResult::Resolved(vec![b, a])
        };
    }
    if sa.pf_reg != sb.pf_reg {
        return if sa.pf_reg > sb.pf_reg {
            TieResult::Resolved(vec![a, b])
        } else {
            TieResult::Resolved(vec![b, a])
        };
    }

    TieResult::Unresolved {
        ordering: vec![TieSlot::Tied(
            vec![a, b],
            format!("Nisu se sreli 2x. Isti diff ({}) i pf ({}). Neresivo.",
                sa.diff_reg, sa.pf_reg),
        )],
    }
}

fn resolve_multi_teams(
    teams: &[Team],
    standings: &HashMap<Team, TeamStats>,
    h2h: &H2HState,
    depth: u32,
) -> TieResult {
    if teams.len() <= 1 { return TieResult::Resolved(teams.to_vec()); }
    if teams.len() == 2 { return resolve_two_teams(teams[0], teams[1], standings, h2h); }
    if depth > 10 {
        let mut sorted = teams.to_vec();
        sorted.sort_by(|a, b| standings.get(b).unwrap().diff_reg.cmp(&standings.get(a).unwrap().diff_reg));
        return TieResult::Resolved(sorted);
    }

    // Check if all pairs met twice
    let all_met = teams.iter().enumerate().all(|(i, &a)| {
        teams[i + 1..].iter().all(|&b| h2h.is_complete(a, b))
    });

    if !all_met {
        // 19.5.1: Not all met twice
        // Check if there are incomplete H2H pairs in this group
        let mut incomplete_pairs: Vec<String> = Vec::new();
        for i in 0..teams.len() {
            for j in (i + 1)..teams.len() {
                if !h2h.is_complete(teams[i], teams[j]) {
                    incomplete_pairs.push(format!("{}-{}", teams[i].code(), teams[j].code()));
                }
            }
        }

        if !incomplete_pairs.is_empty() {
            // Flag with mini-table info
            let mut mini_info = Vec::new();
            for &t in teams {
                let h2h_wins: u32 = teams.iter().filter(|&&o| o != t).map(|&o| h2h.get_wins(t, o)).sum();
                let h2h_diff: i32 = teams.iter().filter(|&&o| o != t).map(|&o| h2h.get_diff(t, o)).sum();
                let s = standings.get(&t).unwrap();
                mini_info.push(format!("{}(h2h_w={},h2h_d={},tot_d={})",
                    t.code(), h2h_wins, h2h_diff, s.diff_reg));
            }

            return TieResult::Unresolved {
                ordering: vec![TieSlot::Tied(
                    teams.to_vec(),
                    format!("Nisu se svi sreli 2x. Nepotpuni H2H parovi: [{}]. Mini-tabela: {}",
                        incomplete_pairs.join(", "), mini_info.join(", ")),
                )],
            };
        }

        // All pairs have played but not twice in both directions → go to global
        return apply_global_criteria_multi(teams, standings);
    }

    // 19.5.2.II: All met twice → H2H mini-league
    let mut h2h_wins_map: HashMap<Team, u32> = HashMap::new();
    let mut h2h_diff_map: HashMap<Team, i32> = HashMap::new();
    for &t in teams {
        let w: u32 = teams.iter().filter(|&&o| o != t).map(|&o| h2h.get_wins(t, o)).sum();
        let d: i32 = teams.iter().filter(|&&o| o != t).map(|&o| h2h.get_diff(t, o)).sum();
        h2h_wins_map.insert(t, w);
        h2h_diff_map.insert(t, d);
    }

    // Group by H2H wins (descending)
    let mut by_wins: BTreeMap<u32, Vec<Team>> = BTreeMap::new();
    for &t in teams {
        by_wins.entry(h2h_wins_map[&t]).or_default().push(t);
    }
    let win_groups: Vec<Vec<Team>> = by_wins.into_iter().rev().map(|(_, ts)| ts).collect();

    if win_groups.len() > 1 {
        return recurse_on_groups(&win_groups, standings, h2h, depth);
    }

    // Same H2H wins → try H2H diff
    let mut by_diff: BTreeMap<i32, Vec<Team>> = BTreeMap::new();
    for &t in teams {
        by_diff.entry(-h2h_diff_map[&t]).or_default().push(t); // negate for descending
    }
    let diff_groups: Vec<Vec<Team>> = by_diff.into_iter().map(|(_, ts)| ts).collect();

    if diff_groups.len() > 1 {
        return recurse_on_groups(&diff_groups, standings, h2h, depth);
    }

    // H2H mini-league completely tied → fall to global
    apply_global_criteria_multi(teams, standings)
}

fn recurse_on_groups(
    groups: &[Vec<Team>],
    standings: &HashMap<Team, TeamStats>,
    h2h: &H2HState,
    depth: u32,
) -> TieResult {
    let mut result_slots = Vec::new();
    for group in groups {
        if group.len() == 1 {
            result_slots.push(TieSlot::Resolved(group[0]));
        } else {
            match resolve_multi_teams(group, standings, h2h, depth + 1) {
                TieResult::Resolved(ordered) => {
                    for t in ordered {
                        result_slots.push(TieSlot::Resolved(t));
                    }
                }
                TieResult::Unresolved { ordering } => {
                    result_slots.extend(ordering);
                }
            }
        }
    }
    if result_slots.iter().all(|s| matches!(s, TieSlot::Resolved(_))) {
        let ordered: Vec<Team> = result_slots.iter().flat_map(|s| s.teams()).collect();
        return TieResult::Resolved(ordered);
    }
    TieResult::Unresolved { ordering: result_slots }
}

fn apply_global_criteria_multi(
    teams: &[Team],
    standings: &HashMap<Team, TeamStats>,
) -> TieResult {
    let mut by_diff: BTreeMap<i32, Vec<Team>> = BTreeMap::new();
    for &t in teams {
        by_diff.entry(-standings.get(&t).unwrap().diff_reg).or_default().push(t);
    }
    let groups: Vec<Vec<Team>> = by_diff.into_iter().map(|(_, ts)| ts).collect();

    let mut result = Vec::new();
    for group in &groups {
        if group.len() == 1 {
            result.push(group[0]);
        } else {
            // Try pf_reg
            let mut by_pf: BTreeMap<i32, Vec<Team>> = BTreeMap::new();
            for &t in group {
                by_pf.entry(-standings.get(&t).unwrap().pf_reg).or_default().push(t);
            }
            let pf_groups: Vec<Vec<Team>> = by_pf.into_iter().map(|(_, ts)| ts).collect();
            for pg in &pf_groups {
                if pg.len() == 1 {
                    result.push(pg[0]);
                } else {
                    // Unresolved
                    let mut slots: Vec<TieSlot> = result.iter().map(|&t| TieSlot::Resolved(t)).collect();
                    let info: Vec<String> = pg.iter().map(|&t| {
                        let s = standings.get(&t).unwrap();
                        format!("{}(d={},pf={})", t.code(), s.diff_reg, s.pf_reg)
                    }).collect();
                    slots.push(TieSlot::Tied(pg.clone(),
                        format!("Isti diff i pf: {}", info.join(", "))));
                    // Add remaining
                    for rg in groups.iter().skip(
                        groups.iter().position(|g| g.iter().any(|t| group.contains(t))).unwrap() + 1
                    ) {
                        for &t in rg {
                            slots.push(TieSlot::Resolved(t));
                        }
                    }
                    return TieResult::Unresolved { ordering: slots };
                }
            }
        }
    }
    TieResult::Resolved(result)
}

// ============================================================================
// SCENARIO RUNNER
// ============================================================================

/// Get the opponent position for a given position in Euroleague playoff bracket
/// Play-in: 7v8, 9v10. QF: 3v6, 4v5. Positions 1&2 play PI winners.
/// Returns None if eliminated (below 10th) or if opponent depends on PI results (pos 1,2)
pub fn opponent_position(pos: u32) -> Option<u32> {
    match pos {
        3 => Some(6),
        4 => Some(5),
        5 => Some(4),
        6 => Some(3),
        7 => Some(8),
        8 => Some(7),
        9 => Some(10),
        10 => Some(9),
        1 | 2 => None, // opponent depends on play-in results
        _ => None,     // eliminated
    }
}

pub fn playoff_round_name(pos: u32) -> &'static str {
    match pos {
        1 | 2 => "Cetvrtfinale (ceka PI pobednika)",
        3..=6 => "Cetvrtfinale",
        7..=10 => "Plej-in",
        _ => "ELIMINISAN",
    }
}

pub struct ScenarioResult {
    /// Full standings: position (1-indexed) -> team
    pub standings: Vec<Team>, // index 0 = 1st place, etc.
    pub red_position: u32,    // exact position if resolved
    pub red_min: u32,
    pub red_max: u32,
    pub has_unresolved_red: bool,
    pub unresolved_info: Vec<String>,
    /// Teams that RED is tied with (if unresolved)
    pub tied_with: Vec<Team>,
}

pub fn run_scenario(
    game_outcomes: &[(usize, GameOutcome)],
    games: &[RemainingGame],
    base: &[TeamStats],
    base_h2h_map: &HashMap<(Team, Team), H2HRecord>,
) -> ScenarioResult {
    // Apply outcomes
    let mut stats: HashMap<Team, TeamStats> = HashMap::new();
    for s in base {
        stats.insert(s.team, s.clone());
    }
    let mut h2h_state = H2HState::new(base_h2h_map);

    for &(gi, outcome) in game_outcomes {
        let game = &games[gi];

        let (winner, loser) = match outcome {
            GameOutcome::HomeWin => (game.home, game.away),
            GameOutcome::AwayWin => (game.away, game.home),
            GameOutcome::H2HLoserWinsSmall | GameOutcome::H2HLoserWinsBig => {
                let info = game.h2h_info.as_ref().unwrap();
                (info.first_loser, info.first_winner)
            }
        };

        if let Some(ws) = stats.get_mut(&winner) {
            ws.gp += 1;
            ws.wins += 1;
            ws.diff_reg += 8;
            ws.pf_reg += 87;
            ws.pa_reg += 79;
        }
        if let Some(ls) = stats.get_mut(&loser) {
            ls.gp += 1;
            ls.losses += 1;
            ls.diff_reg -= 8;
            ls.pf_reg += 79;
            ls.pa_reg += 87;
        }

        h2h_state.apply_game(game, outcome);
    }

    // Build full standings: group by wins, resolve ALL tie groups
    let mut by_wins: BTreeMap<u32, Vec<Team>> = BTreeMap::new();
    for &t in &ALL_TEAMS {
        let s = stats.get(&t).unwrap();
        by_wins.entry(s.wins).or_default().push(t);
    }

    let mut full_standings: Vec<Team> = Vec::with_capacity(20);
    let mut red_min = 0u32;
    let mut red_max = 0u32;
    let mut has_unresolved_red = false;
    let mut unresolved_info: Vec<String> = Vec::new();
    let mut tied_with: Vec<Team> = Vec::new();

    for (_, group) in by_wins.iter().rev() {
        let position = full_standings.len() as u32 + 1;

        if group.len() == 1 {
            if group[0] == Team::RED {
                red_min = position;
                red_max = position;
            }
            full_standings.push(group[0]);
            continue;
        }

        // Resolve tie group
        let result = resolve_tie_group(group, &stats, &h2h_state);

        match result {
            TieResult::Resolved(ordered) => {
                if let Some(red_idx) = ordered.iter().position(|&t| t == Team::RED) {
                    red_min = position + red_idx as u32;
                    red_max = red_min;
                }
                full_standings.extend(ordered);
            }
            TieResult::Unresolved { ordering } => {
                let red_in_group = group.contains(&Team::RED);
                let mut pos = position;
                for slot in &ordering {
                    match slot {
                        TieSlot::Resolved(t) => {
                            if red_in_group && *t == Team::RED {
                                red_min = pos;
                                red_max = pos;
                            }
                            full_standings.push(*t);
                            pos += 1;
                        }
                        TieSlot::Tied(teams, info) => {
                            if red_in_group && teams.contains(&Team::RED) {
                                has_unresolved_red = true;
                                red_min = pos;
                                red_max = pos + teams.len() as u32 - 1;
                                unresolved_info.push(info.clone());
                                tied_with = teams.iter().filter(|&&t| t != Team::RED).cloned().collect();
                            }
                            // For positions assignment, sort tied teams by diff (deterministic fallback)
                            let mut sorted_tied = teams.clone();
                            sorted_tied.sort_by(|a, b| {
                                let sa = stats.get(a).unwrap();
                                let sb = stats.get(b).unwrap();
                                sb.diff_reg.cmp(&sa.diff_reg)
                                    .then(sb.pf_reg.cmp(&sa.pf_reg))
                            });
                            full_standings.extend(sorted_tied);
                            pos += teams.len() as u32;
                        }
                    }
                }
            }
        }
    }

    ScenarioResult {
        standings: full_standings,
        red_position: if red_min == red_max { red_min } else { 0 },
        red_min,
        red_max,
        has_unresolved_red,
        unresolved_info,
        tied_with,
    }
}
