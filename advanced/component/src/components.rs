//! `DiMAS` components example
//! Copyright © 2024 Stephan Kunz

// region:		--- modules
use dimas::prelude::*;
// endregion:	--- modules

#[derive(Debug)]
struct ComponentA {}

impl Component for ComponentA {
    fn id(&self) -> ComponentId {
        ComponentId::from("ComponentA")
    }
}

impl Operational for ComponentA {
	fn state(&self) -> OperationState {
        todo!()
    }

    fn set_state(&mut self, _state: OperationState) {
        todo!()
    }

    fn operationals(&mut self) -> &mut Vec<Box<dyn Operational>> {
        todo!()
    }
}

#[derive(Debug)]
struct ComponentB {}

impl Component for ComponentB {
    fn id(&self) -> ComponentId {
        ComponentId::from("ComponentB")
    }
}

impl Operational for ComponentB {
	fn state(&self) -> OperationState {
        todo!()
    }

    fn set_state(&mut self, _state: OperationState) {
        todo!()
    }

    fn operationals(&mut self) -> &mut Vec<Box<dyn Operational>> {
        todo!()
    }
}

/// Function registers all [`Components`] contained in the library.
/// Needs to be implemented by every library!
#[allow(unsafe_code)]
#[no_mangle]
pub fn register_components(registrar: &mut dyn ComponentRegistrar) -> u32 {
    registrar.register(Box::new(ComponentA {}));
    registrar.register(Box::new(ComponentB {}));
    // A return value of 0 signals success
    0
}

/// Function registers all [`Components`] contained in the library.
/// Needs to be implemented by every library!
#[allow(unsafe_code)]
#[no_mangle]
pub fn unregister_components(registrar: &mut dyn ComponentRegistrar) -> u32 {
    let _ = registrar.deregister(&ComponentId::from("ComponentA"));
    let _ = registrar.deregister(&ComponentId::from("ComponentB"));
    // A return value of 0 signals success
    0
}