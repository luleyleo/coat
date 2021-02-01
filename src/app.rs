use druid::{Application, ExtEventSink, WindowDesc};

use crate::{context::{ContextState, EventCtx, LayoutCtx, PaintCtx}, cx::Cx, id::{ChildCounter, ChildId}, render::AnyRenderObject, tree::{Child, Children}};

pub struct App {
    name: String,
}

impl App {
    pub fn new(name: impl Into<String>) -> Self {
        App { name: name.into() }
    }

    pub fn run(self, app: impl FnMut(&mut Cx) + 'static) -> Result<(), druid::PlatformError> {
        simple_logger::SimpleLogger::new().init().unwrap();

        druid::AppLauncher::with_window(WindowDesc::new(|| AppWidget::new(app))).launch(())
    }
}

struct AppWidget {
    app: Box<dyn FnMut(&mut Cx)>,
    root: Children,
    child_counter: ChildCounter,
    focus_widget: Option<ChildId>,
    ext_event_sink: Option<ExtEventSink>,
}

impl AppWidget {
    pub fn new(app: impl FnMut(&mut Cx) + 'static) -> Self {
        AppWidget {
            app: Box::new(app),
            root: Children::new(),
            child_counter: ChildCounter::new(),
            focus_widget: None,
            ext_event_sink: None,
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
        let ext_handle = ctx.get_external_handle();

        let mut context_state = ContextState {
            ext_handle: &ext_handle,
            window_id: ctx.window_id(),
            window: &ctx.window().clone(),
            text: ctx.text(),
            focus_widget: self.focus_widget,
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
                text: ctx.text(),
                focus_widget: self.focus_widget,
            };
            let mut cx = Cx::new(&mut self.root, &mut context_state, &mut self.child_counter);
            (self.app)(&mut cx);
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

        let mut context_state = ContextState {
            ext_handle: &ext_handle,
            window_id: ctx.window_id(),
            window: &ctx.window().clone(),
            text: ctx.text(),
            focus_widget: self.focus_widget,
        };

        let root = self.root();
        let mut layout_ctx = LayoutCtx {
            state: &mut context_state,
            child_state: &mut root.state,
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
            text: unsafe {
                // TODO: Again, very bad.
                (&mut *(ctx as *mut druid::PaintCtx)).text()
            },
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
        };

        root.object.paint(&mut paint_ctx, &mut root.children);
    }
}
