#[allow(unused_imports)]
use ltrait::{
    color_eyre::eyre::{OptionExt, Result, WrapErr},
    launcher::batcher::Batcher,
    tokio_stream::StreamExt as _,
    ui::{Buffer, UI},
};

pub struct Tui {
    config: TuiConfig,
}

#[derive(Clone)]
pub struct TuiConfig {
    lines: u16,
    selecting: char,
    no_selecting: char,
}

impl TuiConfig {
    pub fn new(lines: u16, selecting: char, no_selecting: char) -> Self {
        Self {
            lines,
            selecting,
            no_selecting,
        }
    }
}

impl Tui {
    pub fn new(config: TuiConfig) -> Self {
        Self { config }
    }
}

/// `<text>`
/// SelectingStatus in above is a char
pub struct TuiEntry {
    pub text: String,
}

impl<'a> UI<'a> for Tui {
    type Context = TuiEntry;

    async fn run<Cusion: 'a + Send>(
        &self,
        mut batcher: Batcher<'a, Cusion, Self::Context>,
    ) -> Result<Cusion> {
        // let mut terminal = ratatui::init_with_options(TerminalOptions {
        //     viewport: Viewport::Inline(self.config.lines),
        // });
        //
        // enable_raw_mode()?;
        // terminal.clear()?;
        //
        // let i = App::new(self.config.clone())
        //     .run(&mut terminal, &mut batcher)
        //     .await?;
        //
        // disable_raw_mode()?;
        // ratatui::restore();
        //
        // batcher.compute_cusion(i)
        todo!()
    }
}

// config: TuiConfig,
// selecting_id: usize,
// buffer: RwLock<Buffer<(TuiEntry, usize)>>,
// id_to_index: Arc<RwLock<FxHashMap<usize, usize>>>,
// index_to_id: Arc<RwLock<FxHashMap<usize, usize>>>,

// (KeyCode::Char('c'), KeyModifiers::CONTROL)
// | (KeyCode::Char('d'), KeyModifiers::CONTROL) => self.exit(),
// (KeyCode::Up, _) | (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
//     let id_to_index = self.id_to_index.read().await;
//     let index = id_to_index.get(&self.selecting_i).unwrap();
//     let new_index = (index + 1).max(id_to_index.len() - 1);
//
//     let index_to_id = self.index_to_id.read().await;
//     self.selecting_i = *index_to_id.get(&new_index).unwrap();
// }
// (KeyCode::Down, _) | (KeyCode::Char('j'), KeyModifiers::CONTROL) => {
//     let id_to_index = self.id_to_index.read().await;
//     let index = id_to_index.get(&self.selecting_i).unwrap();
//     let new_index = index.saturating_sub(1);
//
//     let index_to_id = self.index_to_id.read().await;
//     self.selecting_i = *index_to_id.get(&new_index).unwrap();
// }
