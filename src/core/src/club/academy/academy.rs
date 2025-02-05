use crate::academy::AcademyPlayer;
use crate::club::academy::result::ClubAcademyResult;
use crate::club::academy::settings::AcademySettings;
use crate::context::GlobalContext;
use crate::utils::IntegerUtils;
use crate::PlayerGenerator;
use serde_json::ser::CharEscape::Backspace;

#[derive(Debug)]
pub struct ClubAcademy {
    settings: AcademySettings,
    players: Vec<AcademyPlayer>,
    level: u8,
}

impl ClubAcademy {
    pub fn new(level: u8) -> Self {
        ClubAcademy {
            settings: AcademySettings::default(),
            players: Vec::new(),
            level,
        }
    }

    pub fn simulate(&mut self, ctx: GlobalContext<'_>) -> ClubAcademyResult {
        let mut result = ClubAcademyResult::new();

        if self.players.len() < self.settings.players_count_range.start as usize {
            self.produce_youth_players(ctx);
        }

        let completed_player_ids: Vec<u32> = self
            .players
            .iter()
            .filter(|p| p.completed)
            .map(|p| p.player.id)
            .collect();

        // TODO Filter

        result
    }

    fn produce_youth_players(&mut self, ctx: GlobalContext<'_>) {
        //TODO Country
        let country_id = 0;

        for _ in 0..IntegerUtils::random(5, 15) {
            let generated_player =
                PlayerGenerator::generate(country_id, ctx.simulation.date.date());

            self.players
                .push(AcademyPlayer::from_player(generated_player))
        }
    }
}
