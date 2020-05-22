pub enum Modifier {
    C,
    M,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Char(char),
    Tab,
    Esc,
    F(u8),
    Backspace,
    Del,
    Home,
    End,
    Insert,
    PgUp,
    PgDown,
    Up,
    Down,
    Left,
    Right,
}

pub struct Combination {
    modifiers: Box<[(Modifier, Action)]>,
    final_action: Action,
}

pub use key_map::KeyMap;

mod key_map {
    use super::{Action, Modifier, Combination};
    use crate::input::Input;
    use std::collections::HashMap;
    use std::default::Default;

    pub enum KeyMapping<'map> {
        Alive(LivingMapping<'map>),
        Dead,
    }

    pub struct LivingMapping<'map> {
        map: &'map KeyMap,
        state_id: KeyStateId,
    }

    impl<'map> KeyMapping<'map> {
        pub fn map_cont(&mut self, modifier: Modifier, action: Action) {
            match self {
                KeyMapping::Alive(LivingMapping {
                    map: ref map,
                    state_id: ref mut state_id,
                }) => {
                    let state = &map.key_states[state_id.0];
                    let action_map_id = match modifier {
                        Modifier::M => state.meta_cont,
                        Modifier::C => state.ctrl_cont,
                    };
                    if let Some(new_state_id) = map.action_maps[action_map_id.0].get(action) {
                        *state_id = new_state_id;
                    } else {
                        *self = KeyMapping::Dead
                    }
                }
                KeyMapping::Dead => {}
            }
        }

        pub fn map_final(self, action: Action) -> Option<Input> {
            match self {
                KeyMapping::Alive(LivingMapping {
                    ref map,
                    ref state_id,
                }) => {
                    let KeyState {
                        final_cont: leaf_action_map_id,
                        ..
                    } = map.key_states[state_id.0];
                    map.leaf_action_maps[leaf_action_map_id.0].get(action)
                }
                KeyMapping::Dead => None,
            }
        }
    }

    pub struct KeyMap {
        // TODO Move these all into one allocation
        action_maps: Vec<ActionMap<KeyStateId>>,
        leaf_action_maps: Vec<ActionMap<Input>>,
        key_states: Vec<KeyState>,
    }

    impl KeyMap {
        pub fn register(&mut self, combination: Combination, input: Input) {
            let mut state = KeyStateId::default();
            for (modifier, action) in combination.modifiers.iter() {
                let action_map_id = match modifier {
                    Modifier::M => self.key_states[state.0].meta_cont,
                    Modifier::C => self.key_states[state.0].ctrl_cont,
                };
                state = self.new_state();
                self.action_maps[action_map_id.0].insert(*action, state);
            }
            self.leaf_action_maps[self.key_states[state.0].final_cont.0]
                .insert(combination.final_action, input)
        }

        pub fn new() -> KeyMap {
            KeyMap {
                action_maps: vec![ActionMap::new(), ActionMap::new()],
                leaf_action_maps: vec![ActionMap::new()],
                key_states: vec![KeyState {
                    meta_cont: ActionMapId(0),
                    ctrl_cont: ActionMapId(1),
                    final_cont: LeafActionMapId(0),
                }],
            }
        }

	pub fn map<'map>(&'map self) -> KeyMapping<'map>{
	    KeyMapping::Alive(LivingMapping{
		map: &self,
		state_id: KeyStateId::default(),
	    })
	}

        fn new_state(&mut self) -> KeyStateId {
            self.action_maps.push(ActionMap::new());
            self.action_maps.push(ActionMap::new());
            self.leaf_action_maps.push(ActionMap::new());
            self.key_states.push(KeyState {
                meta_cont: ActionMapId(self.action_maps.len() - 1),
                ctrl_cont: ActionMapId(self.action_maps.len() - 2),
                final_cont: LeafActionMapId(self.leaf_action_maps.len() - 1),
            });
            KeyStateId(self.key_states.len() - 1)
        }
    }

    #[derive(Clone, Copy)]
    struct ActionMapId(usize);
    #[derive(Clone, Copy)]
    struct LeafActionMapId(usize);

    struct ActionMap<Item: Copy> {
        // TODO Make this a mostly in place datastructure that does not heap allocate.
        map: HashMap<Action, Item>,
    }

    impl<Item: Copy> ActionMap<Item> {
        fn get(&self, key: Action) -> Option<Item> {
            self.map.get(&key).map(|i| *i)
        }

        fn insert(&mut self, key: Action, value: Item) {
            self.map.insert(key, value);
        }

        fn new() -> ActionMap<Item> {
            ActionMap {
                map: HashMap::new(),
            }
        }
    }

    #[derive(Clone, Copy)]
    struct KeyStateId(usize);

    impl Default for KeyStateId {
        fn default() -> KeyStateId {
            KeyStateId(0)
        }
    }

    struct KeyState {
        meta_cont: ActionMapId,
        ctrl_cont: ActionMapId,
        final_cont: LeafActionMapId,
    }

    #[cfg(test)]
    mod tests {
	use super::*;
	
	#[test]
	fn basic_store_and_reteive(){
	    let mut km = KeyMap::new();
	    km.register(Combination{modifiers: Box::new([(Modifier::C, Action::Char('x'))]), final_action: Action::Char('s')}, Input::DUMMY);
	    let mut mp = km.map();
	    mp.map_cont(Modifier::C, Action::Char('x'));
	    assert_eq!(mp.map_final(Action::Char('s')), Some(Input::DUMMY));
	}
    }
}
