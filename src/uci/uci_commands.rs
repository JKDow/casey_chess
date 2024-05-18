use std::fmt;

#[derive(Debug, PartialEq)]
pub enum UciGuiToEngine {
    Uci,
    Debug(String),
    IsReady,
    SetOption(String),
    UciNewGame,
    Position(String),
    Go(String),
    Stop,
    PonderHit,
    Quit,
}

impl UciGuiToEngine {
    pub fn from_string(input: &str) -> Option<UciGuiToEngine> {
        let mut parts = input.split_whitespace();
        match parts.next() {
            Some("uci") => Some(UciGuiToEngine::Uci),
            Some("debug") => Some(UciGuiToEngine::Debug(parts.collect::<Vec<&str>>().join(" "))),
            Some("isready") => Some(UciGuiToEngine::IsReady),
            Some("setoption") => Some(UciGuiToEngine::SetOption(parts.collect::<Vec<&str>>().join(" "))),
            Some("ucinewgame") => Some(UciGuiToEngine::UciNewGame),
            Some("position") => Some(UciGuiToEngine::Position(parts.collect::<Vec<&str>>().join(" "))),
            Some("go") => Some(UciGuiToEngine::Go(parts.collect::<Vec<&str>>().join(" "))),
            Some("stop") => Some(UciGuiToEngine::Stop),
            Some("ponderhit") => Some(UciGuiToEngine::PonderHit),
            Some("quit") => Some(UciGuiToEngine::Quit),
            _ => None,
        }
    }
}

pub enum UciEngineToGui {
    Id(String),
    UciOk,
    ReadyOk,
    BestMove(String),
    CopyProtection(String),
    Registration(String),
    Info(String),
    Option(String),
}

impl UciEngineToGui {
    pub fn id_name(name: &str) -> UciEngineToGui {
        UciEngineToGui::Id(format!("name {}", name))
    }

    pub fn id_author(author: &str) -> UciEngineToGui {
        UciEngineToGui::Id(format!("author {}", author))
    }

    pub fn uci_ok() -> UciEngineToGui {
        UciEngineToGui::UciOk
    }

    pub fn ready_ok() -> UciEngineToGui {
        UciEngineToGui::ReadyOk
    }

    pub fn best_move(mov: &str) -> UciEngineToGui {
        UciEngineToGui::BestMove(mov.to_string())
    }

    pub fn copy_protection(s: &str) -> UciEngineToGui {
        UciEngineToGui::CopyProtection(s.to_string())
    }

    pub fn registration(s: &str) -> UciEngineToGui {
        UciEngineToGui::Registration(s.to_string())
    }

    pub fn info(s: &str) -> UciEngineToGui {
        UciEngineToGui::Info(s.to_string())
    }

    pub fn option(s: &str) -> UciEngineToGui {
        UciEngineToGui::Option(s.to_string())
    }
}

impl fmt::Display for UciEngineToGui {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UciEngineToGui::Id(s) => write!(f, "id {}", s),
            UciEngineToGui::UciOk => write!(f, "uciok"),
            UciEngineToGui::ReadyOk => write!(f, "readyok"),
            UciEngineToGui::BestMove(s) => write!(f, "bestmove {}", s),
            UciEngineToGui::CopyProtection(s) => write!(f, "copyprotection {}", s),
            UciEngineToGui::Registration(s) => write!(f, "registration {}", s),
            UciEngineToGui::Info(s) => write!(f, "info {}", s),
            UciEngineToGui::Option(s) => write!(f, "option {}", s),
        }
    }
}
