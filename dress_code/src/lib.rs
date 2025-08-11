#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}
pub fn choose_outfit(
    formality_level: Option<u32>,
    invitation_message: Result<&str, &str>
) -> Outfit {
    if formality_level.is_none() && invitation_message.is_ok() {
        return Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Fedora,
        };
    }
    if formality_level.is_none() && invitation_message.is_err() {
        return Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Baseball,
        };
    }
    let jacket = match formality_level {
        Some(n) => if n > 0 { Jacket::White } else { Jacket::Black }
        None => Jacket::Flowers,
    };
    let hat = match invitation_message {
        Ok(_) => Hat::Fedora,
        Err(_) => Hat::Snapback,
        _ => Hat::Baseball,
    };

    Outfit { jacket, hat }
}
