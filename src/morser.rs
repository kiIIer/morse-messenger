use std::fmt::{Display, Formatter};
tonic::include_proto!("morser");

impl Display for Signal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Signal[{}]", self.state)
    }
}
