use super::uci_commands::UciGuiToEngine;


#[derive(Debug, PartialEq)]
pub enum HandlerTx {
    NewFen(String),
    StartingPosition(String),
    StartSearch,
    StopSearch,
    MakeMove(String),
}

#[derive(Debug, PartialEq)]
pub enum HandlerRx {
    EngineMsg(EngineMsg),
    GuiMsg(UciGuiToEngine),
}

#[derive(Debug, PartialEq)]
pub enum EngineMsg {
    PositionSet,
    CurrentBestMove(String),
    FinalBestMove(String),
}
