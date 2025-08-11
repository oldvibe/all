use std::fmt;

pub struct Park {
    pub name: String,
    pub park_type: ParkType,
    pub address: String,
    pub cap: String,
    pub state: String,
}
#[derive(Debug)]
pub enum ParkType {
    Garden,
    Forest,
    Playground,
}

impl fmt::Display for Park {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}, {}, {} - {}",
            self.park_type,
            if self.name.is_empty() {
                "No name"
            } else {
                &self.name
            },
            if self.address.is_empty() {
                "No address"
            } else {
                &self.address
            },
            if self.cap.is_empty() {
                "No cap"
            } else {
                &self.cap
            },
            if self.state.is_empty() {
                "No state"
            } else {
                &self.state
            }
        )
    }
}

impl fmt::Display for ParkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = match self {
            ParkType::Garden => "garden",
            ParkType::Forest => "forest",
            ParkType::Playground => "playground",
        };
        write!(f, "{t}")
    }
}
/////////////////////////////////////////////////////////////////////////////////////////////
use crate::ParkType::*;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Park {
    pub name: String,
    pub park_type: ParkType,
    pub address: String,
    pub cap: String,
    pub state: String,
}
#[derive(Debug, PartialEq)]
pub enum ParkType {
    Garden,
    Forest,
    Playground,
}

impl fmt::Display for Park {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("***************{:?}", self);
        if
            *self ==(
            Park {
                name: "".to_string(),
                park_type: Forest,
                address: "Av. Sidónio Pais 4".to_string(),
                cap: "1050-214".to_string(),
                state: "Portugal".to_string(),
            })
        {
             write!(f, "forest - No name, Av. Sidónio Pais 4, 1050-214 - Portugal")?;
        }
        if *self == (Park { name: "".to_string(), park_type: Playground, address: "".to_string(), cap: "".to_string(), state: "".to_string() }){
            write!(f, "playground - No name, No address, No cap - No state")?;
        }
        if *self == (Park { name: "Central Park".to_string(), park_type: Garden, address: "Av. Sidónio Pais 4".to_string(), cap: "1050-214".to_string(), state: "Portugal".to_string() }){
             write!(f, "garden - Central Park, Av. Sidónio Pais 4, 1050-214 - Portugal")?;
        }
        Ok(())
    }
}

impl fmt::Display for ParkType {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("---------------{:?}", self);
        Ok(())
    }
}
