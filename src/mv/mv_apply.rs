use crate::pos::pos::Pos;

pub fn apply(p: &Pos, mv: u16) -> Option<Pos> {
    let mut new_pos = p.clone();

    if legal_pos(&new_pos) {
        return Some(new_pos);
    } else {
        return None;
    }
}

pub fn legal_pos(p: &Pos) -> bool {
    // This is turned around since the opponents king is allowed to be in check
    // Since the switching of the active player has already happend
    // The only illegal thing would be, when the active player could capture the king
    let king_bb = if p.active == 1 { p.bk } else { p.wk };
    let attack_bb = if p.active == 1 { p.wattack } else { p.battack };
    king_bb & attack_bb != 0
}
