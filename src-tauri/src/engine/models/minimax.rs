use super::EngineModel;

/// MiniMax Code's model catalog, mirroring the CLI's own /model menu as
/// observed on a live 0.5.1 install (M3 lead, M2.7 lines behind). The
/// catalog is deliberately non-authoritative: new MiniMax models appear
/// faster than this list ships, so custom ids stay selectable — the ACP
/// driver resolves each pick against the session's own advertised options
/// and keeps the CLI default (with a notice) when there is no match.
pub(super) fn minimax_local_models() -> Vec<EngineModel> {
    vec![
        EngineModel {
            id: "minimax/MiniMax-M3".to_string(),
            name: Some("MiniMax-M3".to_string()),
            description: None,
            provider: "minimax".to_string(),
            context_window: None,
        },
        EngineModel {
            id: "minimax/MiniMax-M2.7".to_string(),
            name: Some("MiniMax-M2.7".to_string()),
            description: None,
            provider: "minimax".to_string(),
            context_window: Some(200_000),
        },
        EngineModel {
            id: "minimax/MiniMax-M2.7-highspeed".to_string(),
            name: Some("MiniMax-M2.7-highspeed".to_string()),
            description: None,
            provider: "minimax".to_string(),
            context_window: Some(200_000),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_leads_with_the_newest_model() {
        let models = minimax_local_models();
        assert_eq!(models[0].id, "minimax/MiniMax-M3");
        assert!(models.iter().all(|model| model.id.starts_with("minimax/")));
        assert_eq!(models[1].context_window, Some(200_000));
    }
}
