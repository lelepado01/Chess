
use amethyst::{
    prelude::*, 
    ecs::{Component, DenseVecStorage, Dispatcher, DispatcherBuilder}, 
};

use crate::systems::selection::{BlackSelectionSystem, WhiteSelectionSystem}; 
use crate::systems::movement::{BlackMovementSystem, WhiteMovementSystem}; 
use crate::systems::eating::{BlackEatingSystem, WhiteEatingSystem};

pub struct BlackTurnState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
}
pub struct WhiteTurnState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
}

#[derive(Default)]
pub struct TurnProgress {
    pub moved : bool
}

impl Component for TurnProgress {
    type Storage = DenseVecStorage<Self>;
}

impl<'a, 'b> SimpleState for WhiteTurnState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(mut turn_progress) = data.world.try_fetch_mut::<TurnProgress>() {
            turn_progress.moved = false;
        }

        let mut dispatcher_builder = DispatcherBuilder::new(); 
        dispatcher_builder.add(WhiteSelectionSystem, "white_selection_system", &[]);
        dispatcher_builder.add(WhiteMovementSystem, "white_movement_system", &[]);
        dispatcher_builder.add(BlackEatingSystem, "black_eating_system", &[]);

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(data.world);
        self.dispatcher = Some(dispatcher);
    }

    fn update(&mut self, data : &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &data.world;

        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        if let Some(mut turn_progress) = world.try_fetch_mut::<TurnProgress>(){
            if turn_progress.moved {
                turn_progress.moved = false;
                Trans::Switch(Box::new(BlackTurnState{dispatcher: None}))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

impl<'a, 'b> SimpleState for BlackTurnState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(mut turn_progress) = data.world.try_fetch_mut::<TurnProgress>() {
            turn_progress.moved = false;
        }

        let mut dispatcher_builder = DispatcherBuilder::new(); 

        dispatcher_builder.add(BlackSelectionSystem, "black_selection_system", &[]);
        dispatcher_builder.add(BlackMovementSystem, "black_movement_system", &[]);
        dispatcher_builder.add(WhiteEatingSystem, "white_eating_system", &[]);


        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(data.world);
        self.dispatcher = Some(dispatcher);
    }

    fn update(&mut self, data : &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &data.world;

        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        if let Some(mut turn_progress) = world.try_fetch_mut::<TurnProgress>() {
            if turn_progress.moved {
                turn_progress.moved = false;
                Trans::Switch(Box::new(WhiteTurnState{dispatcher : None}))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}