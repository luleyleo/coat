use crate::{
    context::{ContextState, EventCtx, LayoutCtx, PaintCtx},
    id::{ChildCounter, ChildId},
    kurbo::Point,
    tree::{Child, Children},
    ui::Ui,
};
use druid::{ExtEventSink, WindowDesc};

pub struct App {
    name: String,
}

impl App {
    pub fn new(name: impl Into<String>) -> Self {
        App { name: name.into() }
    }

    pub fn run(self, app: impl FnMut(&mut Ui) + 'static) -> Result<(), druid::PlatformError> {
        simple_logger::SimpleLogger::new().init().unwrap();

        let window = WindowDesc::new(|| AppWidget::new(app)).title(self.name);
        druid::AppLauncher::with_window(window).launch(())
    }
}

struct AppWidget {
    app: Box<dyn FnMut(&mut Ui)>,
    root: Children,
    child_counter: ChildCounter,
    focus_widget: Option<ChildId>,
    ext_event_sink: Option<ExtEventSink>,
    mouse_pos: Option<Point>,
}

impl AppWidget {
    pub fn new(app: impl FnMut(&mut Ui) + 'static) -> Self {
        AppWidget {
            app: Box::new(app),
            root: Children::new(),
            child_counter: ChildCounter::new(),
            focus_widget: None,
            ext_event_sink: None,
            mouse_pos: None,
        }
    }

    fn root(&mut self) -> &mut Child {
        &mut self.root.renders[0]
    }
}

type AppWidgetData = ();

impl druid::Widget<AppWidgetData> for AppWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppWidgetData,
        env: &druid::Env,
    ) {
        ctx.set_active(true);
        ctx.request_focus();
        let ext_handle = ctx.get_external_handle();

        match event {
            druid::Event::MouseMove(event)
            | druid::Event::MouseUp(event)
            | druid::Event::MouseDown(event) => {
                self.mouse_pos = Some(event.pos);
            }
            _ => {}
        }

        let focus_widget = self.focus_widget;

        let mut context_state = ContextState {
            ext_handle: &ext_handle,
            window_id: ctx.window_id(),
            window: &ctx.window().clone(),
            text: ctx.text().clone(),
            focus_widget,
        };

        let root = self.root();
        let mut event_ctx = EventCtx {
            state: &mut context_state,
            child_state: &mut root.state,
            is_handled: false,
            is_root: true,
        };

        root.object.event(&mut event_ctx, event, &mut root.children);
        ctx.request_paint_rect(root.state.invalid.bounding_box());
        ctx.request_layout();

        let old_focus_widget = self.focus_widget;
        if let Some(focus_change) = self.root().state.request_focus {
            match focus_change {
                crate::tree::FocusChange::Resign => self.focus_widget = None,
                crate::tree::FocusChange::Focus(id) => self.focus_widget = Some(id),
                crate::tree::FocusChange::Next => {}
                crate::tree::FocusChange::Previous => {}
            }
        }
        if self.focus_widget != old_focus_widget {
            let new_focus_widget = self.focus_widget;
            self.root().update_focus(new_focus_widget);
        }

        let mut needs_update = self.root().needs_update();
        while needs_update {
            needs_update = self.root().needs_update();

            let ext_handle = ctx.get_external_handle();
            let mut context_state = ContextState {
                ext_handle: &ext_handle,
                window_id: ctx.window_id(),
                window: &ctx.window().clone(),
                text: ctx.text().clone(),
                focus_widget,
            };
            let mut cx = Ui::new(&mut self.root, &mut context_state, &mut self.child_counter);
            (self.app)(&mut cx);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &AppWidgetData,
        env: &druid::Env,
    ) {
        if matches!(event, druid::LifeCycle::WidgetAdded) {
            let ext_handle = ctx.get_external_handle();
            self.ext_event_sink = Some(ext_handle.clone());

            let mut context_state = ContextState {
                ext_handle: &ext_handle,
                window_id: ctx.window_id(),
                window: &ctx.window().clone(),
                text: ctx.text().clone(),
                focus_widget: self.focus_widget,
            };
            let mut cx = Ui::new(&mut self.root, &mut context_state, &mut self.child_counter);
            (self.app)(&mut cx);
        }
        if matches!(event, druid::LifeCycle::HotChanged(false)) {
            self.mouse_pos = None;
        }
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &AppWidgetData,
        data: &AppWidgetData,
        env: &druid::Env,
    ) {
        // Nothing to do
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &AppWidgetData,
        env: &druid::Env,
    ) -> druid::Size {
        let ext_handle = ctx.get_external_handle();
        let mouse_pos = self.mouse_pos;

        let mut context_state = ContextState {
            ext_handle: &ext_handle,
            window_id: ctx.window_id(),
            window: &ctx.window().clone(),
            text: ctx.text().clone(),
            focus_widget: self.focus_widget,
        };

        let root = self.root();
        let mut layout_ctx = LayoutCtx {
            state: &mut context_state,
            child_state: &mut root.state,
            mouse_pos,
            env,
        };

        root.state.size = root.object.layout(&mut layout_ctx, bc, &mut root.children);
        root.state.size
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppWidgetData, env: &druid::Env) {
        let ext_handle = self.ext_event_sink.clone().unwrap();

        let mut context_state = ContextState {
            ext_handle: &ext_handle,
            window_id: ctx.window_id(),
            window: &ctx.window().clone(),
            text: ctx.text().clone(),
            focus_widget: self.focus_widget,
        };

        let root = self.root();
        let mut paint_ctx = PaintCtx {
            state: &mut context_state,
            child_state: &mut root.state,
            z_ops: Vec::new(),
            region: ctx.region().clone(),
            depth: ctx.depth(),
            render_ctx: ctx.render_ctx,
            env,
        };

        root.object.paint(&mut paint_ctx, &mut root.children);
    }
}
