use std::io::{self, Result};

use rsh_protocol::{
    engine::{EngineState, Stack},
    PipelineData, Value,
};
use ratatui::layout::Rect;

use crate::{
    rsh_common::{collect_pipeline, has_simple_value, run_command_with_value},
    pager::Frame,
    views::{Layout, Orientation, Preview, RecordView, View, ViewConfig},
};

use super::{HelpExample, HelpManual, ViewCommand};

#[derive(Debug, Default, Clone)]
pub struct RshCmd {
    command: String,
}

impl RshCmd {
    pub fn new() -> Self {
        Self {
            command: String::new(),
        }
    }

    pub const NAME: &'static str = "rsh";
}

impl ViewCommand for RshCmd {
    type View = RshView<'static>;

    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn usage(&self) -> &'static str {
        ""
    }

    fn help(&self) -> Option<HelpManual> {
        let examples = vec![
            HelpExample::new(
                "where type == 'file'",
                "Filter data to show only rows whose type is 'file'",
            ),
            HelpExample::new(
                "get scope.examples",
                "Navigate to a deeper value inside the data",
            ),
            HelpExample::new("open Cargo.toml", "Open a Cargo.toml file"),
        ];

        Some(HelpManual {
            name: "rsh",
            description:
                "Run a rsh command. The data currently being explored is piped into it.",
            examples,
            arguments: vec![],
            input: vec![],
            config_options: vec![],
        })
    }

    fn parse(&mut self, args: &str) -> Result<()> {
        self.command = args.trim().to_owned();

        Ok(())
    }

    fn spawn(
        &mut self,
        engine_state: &EngineState,
        stack: &mut Stack,
        value: Option<Value>,
    ) -> Result<Self::View> {
        let value = value.unwrap_or_default();

        let pipeline = run_command_with_value(&self.command, &value, engine_state, stack)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let is_record = matches!(pipeline, PipelineData::Value(Value::Record { .. }, ..));

        let (columns, values) = collect_pipeline(pipeline);

        if let Some(value) = has_simple_value(&values) {
            let text = value.into_abbreviated_string(&engine_state.config);
            return Ok(RshView::Preview(Preview::new(&text)));
        }

        let mut view = RecordView::new(columns, values);

        if is_record {
            view.set_orientation_current(Orientation::Left);
        }

        Ok(RshView::Records(view))
    }
}

pub enum RshView<'a> {
    Records(RecordView<'a>),
    Preview(Preview),
}

impl View for RshView<'_> {
    fn draw(&mut self, f: &mut Frame, area: Rect, cfg: ViewConfig<'_>, layout: &mut Layout) {
        match self {
            RshView::Records(v) => v.draw(f, area, cfg, layout),
            RshView::Preview(v) => v.draw(f, area, cfg, layout),
        }
    }

    fn handle_input(
        &mut self,
        engine_state: &EngineState,
        stack: &mut Stack,
        layout: &Layout,
        info: &mut crate::pager::ViewInfo,
        key: crossterm::event::KeyEvent,
    ) -> Option<crate::pager::Transition> {
        match self {
            RshView::Records(v) => v.handle_input(engine_state, stack, layout, info, key),
            RshView::Preview(v) => v.handle_input(engine_state, stack, layout, info, key),
        }
    }

    fn show_data(&mut self, i: usize) -> bool {
        match self {
            RshView::Records(v) => v.show_data(i),
            RshView::Preview(v) => v.show_data(i),
        }
    }

    fn collect_data(&self) -> Vec<crate::rsh_common::RshText> {
        match self {
            RshView::Records(v) => v.collect_data(),
            RshView::Preview(v) => v.collect_data(),
        }
    }

    fn exit(&mut self) -> Option<Value> {
        match self {
            RshView::Records(v) => v.exit(),
            RshView::Preview(v) => v.exit(),
        }
    }

    fn setup(&mut self, config: ViewConfig<'_>) {
        match self {
            RshView::Records(v) => v.setup(config),
            RshView::Preview(v) => v.setup(config),
        }
    }
}
