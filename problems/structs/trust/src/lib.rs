#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AgentMove {
    #[default]
    Cooperated,
    Cheated,
}

pub struct Game {
    left_agent: Box<dyn Agent>,
    right_agent: Box<dyn Agent>,
    left_score: i32,
    right_score: i32,
}

impl Game {
    pub fn new(left: Box<dyn Agent>, right: Box<dyn Agent>) -> Self {
        Self {
            left_agent: left,
            right_agent: right,
            left_score: 0,
            right_score: 0,
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left_score
    }

    pub fn right_score(&self) -> i32 {
        self.right_score
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        let left_move: AgentMove = self.left_agent.play_round();
        let right_move: AgentMove = self.right_agent.play_round();

        self.left_agent.set_prev_opponent_move(right_move);
        self.right_agent.set_prev_opponent_move(left_move);

        if left_move == AgentMove::Cooperated && right_move == AgentMove::Cooperated {
            self.left_score += 2;
            self.right_score += 2;
            RoundOutcome::BothCooperated
        } else if left_move == AgentMove::Cooperated && right_move == AgentMove::Cheated {
            self.left_score -= 1;
            self.right_score += 3;
            RoundOutcome::RightCheated
        } else if left_move == AgentMove::Cheated && right_move == AgentMove::Cooperated {
            self.left_score += 3;
            self.right_score -= 1;
            RoundOutcome::LeftCheated
        } else {
            RoundOutcome::BothCheated
        }
    }
}

pub trait Agent {
    fn play_round(&mut self) -> AgentMove;

    fn set_prev_opponent_move(&mut self, opponent_move: AgentMove);
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {
    pub prev_opponent_move: AgentMove,
}

impl Agent for CheatingAgent {
    fn play_round(&mut self) -> AgentMove {
        AgentMove::Cheated
    }

    fn set_prev_opponent_move(&mut self, opponent_move: AgentMove) {
        self.prev_opponent_move = opponent_move;
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {
    pub prev_opponent_move: AgentMove,
}

impl Agent for CooperatingAgent {
    fn play_round(&mut self) -> AgentMove {
        AgentMove::Cooperated
    }

    fn set_prev_opponent_move(&mut self, opponent_move: AgentMove) {
        self.prev_opponent_move = opponent_move;
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct GrudgerAgent {
    pub prev_opponent_move: AgentMove,
    is_deceived: bool,
}

impl Agent for GrudgerAgent {
    fn play_round(&mut self) -> AgentMove {
        if self.prev_opponent_move == AgentMove::Cheated {
            self.is_deceived = true;
        }

        if self.is_deceived {
            return AgentMove::Cheated;
        }

        AgentMove::Cooperated
    }

    fn set_prev_opponent_move(&mut self, opponent_move: AgentMove) {
        self.prev_opponent_move = opponent_move;
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CopycatAgent {
    pub prev_opponent_move: AgentMove,
    is_first_move: bool,
}

impl Agent for CopycatAgent {
    fn play_round(&mut self) -> AgentMove {
        if !self.is_first_move {
            self.is_first_move = true;
            return AgentMove::Cooperated;
        }

        self.prev_opponent_move
    }

    fn set_prev_opponent_move(&mut self, opponent_move: AgentMove) {
        self.prev_opponent_move = opponent_move;
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct DetectiveAgent {
    pub prev_opponent_move: AgentMove,
    is_deceived: bool,
    is_always_cheat: bool,
    index: usize,
}

impl Agent for DetectiveAgent {
    fn play_round(&mut self) -> AgentMove {
        if self.prev_opponent_move == AgentMove::Cheated {
            self.is_deceived = true;
        }

        let moves: Vec<AgentMove> = vec![
            AgentMove::Cooperated,
            AgentMove::Cheated,
            AgentMove::Cooperated,
            AgentMove::Cooperated,
        ];
        if self.index < 4 {
            self.index += 1;
            return moves[self.index - 1];
        }

        if !self.is_deceived {
            self.is_always_cheat = true;
        }

        if self.is_always_cheat {
            return AgentMove::Cheated;
        }

        self.prev_opponent_move
    }

    fn set_prev_opponent_move(&mut self, opponent_move: AgentMove) {
        self.prev_opponent_move = opponent_move;
    }
}
