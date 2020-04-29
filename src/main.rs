use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    input::{InputBundle, StringBindings}
};

mod board;
mod camera;
mod highlight_circle;
use highlight_circle::initialize_highlight;
mod systems; 
use systems::highlight::HighlightSystem;
mod pieces;
use pieces::piece_utils::initialize_pieces;
mod states;
use states::TurnProgress; 

struct LoadState;

impl SimpleState for LoadState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<board::Board>();
        world.register::<pieces::piece_types::BlackPiece>();
        world.register::<pieces::piece_types::WhitePiece>();

        camera::initialize_camera(world);
        board::initialize_board(world);
        initialize_highlight(world);
        initialize_pieces(world);

        let turn_progress = TurnProgress {
            moved : false
        };
        world.insert(turn_progress); 
    }

    fn update(&mut self, _data : &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::Push(Box::new(states::WhiteTurnState{dispatcher: None}))
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let mouse_bindings_path = assets_dir.join("mouse_bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(mouse_bindings_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([1.0, 1.0, 1.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(HighlightSystem, "highlight_system", &[]);

    let mut game = Application::new(assets_dir, LoadState, game_data)?;
    game.run();

    Ok(())
}
