use crate::{FocusPolicy, Interaction, Node};
use bevy_ecs::{component::Component, reflect::ReflectComponent};
use bevy_reflect::{std_traits::ReflectDefault, Reflect};

#[cfg(feature = "serialize")]
use serde::{Serialize, Deserialize}; 

/// Marker struct for buttons
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[reflect(Component, Default, Debug, PartialEq, Clone)]
#[require(Node, FocusPolicy::Block, Interaction)]
pub struct Button;
