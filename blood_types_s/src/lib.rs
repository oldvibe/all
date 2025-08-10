#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let abo_ok = match self.antigen {
            Antigen::O => matches!(other.antigen, Antigen::O),
            Antigen::A => matches!(other.antigen, Antigen::O | Antigen::A),
            Antigen::B => matches!(other.antigen, Antigen::O | Antigen::B),
            Antigen::AB => true,
        };

        let rh_ok = match self.rh_factor {
            RhFactor::Negative => matches!(other.rh_factor, RhFactor::Negative),
            RhFactor::Positive => true,
        };

        abo_ok && rh_ok
    }

    fn all_types() -> Vec<BloodType> {
        let antigens = [Antigen::O, Antigen::A, Antigen::B, Antigen::AB];
        let rhs = [RhFactor::Negative, RhFactor::Positive];
        let mut result = vec![];
        for a in &antigens {
            for r in &rhs {
                result.push(BloodType {
                    antigen: a.clone(),
                    rh_factor: r.clone(),
                });
            }
        }
        result
    }

    pub fn donors(&self) -> Vec<Self> {
        Self::all_types()
            .into_iter()
            .filter(|bt| self.can_receive_from(bt))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        Self::all_types()
            .into_iter()
            .filter(|bt| bt.can_receive_from(self))
            .collect()
    }
}

