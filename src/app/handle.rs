use crate::{
    app::win_handler::RUN_COMMANDS_TOKEN,
    context::{ContextState, EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx},
    event::{Event, Handled, InternalEvent, InternalLifeCycle, LifeCycle},
    ext_event::ExtEventHost,
    id::{ChildCounter, ChildId, WindowId},
    kurbo::{Point, Size},
    piet::{Color, Piet, RenderContext},
    shell::{self, Cursor, Region},
    tree::{Child, ChildState, Children, FocusChange},
    ui::Ui,
    BoxConstraints,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub(crate) struct AppHandle {
    inner: Rc<RefCell<AppHandleInner>>,
}

struct AppHandleInner {
    app: Box<dyn FnMut(&mut Ui)>,
    shell_app: shell::Application,
    handle: Option<shell::WindowHandle>,
    ext_event_host: ExtEventHost,
    child_counter: ChildCounter,
    children: Children,
    last_mouse_pos: Option<Point>,
    focus_widget: Option<ChildId>,
    env: crate::env::Env,
    invalid: Region,
    size: Size,
}

impl AppHandleInner {
    fn handle(&self) -> &shell::WindowHandle {
        self.handle.as_ref().unwrap()
    }

    fn root(&mut self) -> &mut Child {
        &mut self.children.renders[0]
    }

    fn layout(&mut self) {
        let mut context_state = ContextState {
            ext_host: &self.ext_event_host,
            window_id: WindowId::illigal(),
            window: self.handle.as_ref().unwrap(),
            text: self.handle.as_ref().unwrap().text(),
            focus_widget: self.focus_widget,
        };

        let root = &mut self.children.renders[0];
        let mut layout_ctx = LayoutCtx {
            state: &mut context_state,
            child_state: &mut root.state,
            mouse_pos: self.last_mouse_pos,
            env: &self.env,
        };

        let bc = BoxConstraints::tight(self.size);

        let root_size = root.object.layout(&mut layout_ctx, &bc, &mut root.children);
        root.state.size = root_size;
    }
}

impl AppHandleInner {
    pub fn initialize(&mut self) {}

    pub fn connect(&mut self, handle: &shell::WindowHandle) {
        self.handle = Some(handle.clone());

        let mut context_state = ContextState {
            ext_host: &self.ext_event_host,
            window_id: WindowId::illigal(),
            window: self.handle.as_ref().unwrap(),
            text: self.handle.as_ref().unwrap().text(),
            focus_widget: self.focus_widget,
        };
        let mut ui = Ui::new(
            &mut self.children,
            &mut context_state,
            &mut self.child_counter,
        );
        (self.app)(&mut ui);
        self.layout();

        handle.show();
    }

    pub fn prepare_paint(&mut self) {
        self.layout();
    }

    pub fn paint(&mut self, piet: &mut Piet, invalid: &Region) {
        piet.fill(invalid.bounding_box(), &Color::rgb(0.1, 0.1, 0.1));

        let mut context_state = ContextState {
            ext_host: &self.ext_event_host,
            window_id: WindowId::illigal(),
            window: self.handle.as_ref().unwrap(),
            text: self.handle.as_ref().unwrap().text(),
            focus_widget: self.focus_widget,
        };

        let root = &mut self.children.renders[0];
        let mut paint_ctx = PaintCtx {
            state: &mut context_state,
            render_ctx: piet,
            child_state: &mut root.state,
            z_ops: Vec::new(),
            region: invalid.clone(),
            depth: 0,
            env: &self.env,
        };

        root.object.paint(&mut paint_ctx, &mut root.children);

        let mut z_ops = std::mem::take(&mut paint_ctx.z_ops);
        z_ops.sort_by_key(|k| k.z_index);

        for z_op in z_ops.into_iter() {
            paint_ctx.with_child_ctx(invalid.clone(), |ctx| {
                ctx.with_save(|ctx| {
                    ctx.render_ctx.transform(z_op.transform);
                    (z_op.paint_func)(ctx);
                });
            });
        }
    }

    pub fn event(&mut self, event: Event) -> Handled {
        match &event {
            Event::WindowCloseRequested => self.handle().close(),
            Event::WindowSize(size) => self.size = *size,
            Event::MouseDown(e) | Event::MouseUp(e) | Event::MouseMove(e) | Event::Wheel(e) => {
                self.last_mouse_pos = Some(e.pos)
            }
            Event::Internal(InternalEvent::MouseLeave) => self.last_mouse_pos = None,
            _ => (),
        }

        let mut child_state = ChildState::new(self.root().state.id, Some(self.size));
        let is_handled = {
            let mut context_state = ContextState {
                ext_host: &self.ext_event_host,
                window_id: WindowId::illigal(),
                window: self.handle.as_ref().unwrap(),
                text: self.handle.as_ref().unwrap().text(),
                focus_widget: self.focus_widget,
            };
            let mut ctx = EventCtx {
                state: &mut context_state,
                child_state: &mut child_state,
                is_handled: false,
                is_root: true,
            };

            self.children.renders[0].event(&mut ctx, &event);
            Handled::from(ctx.is_handled)
        };

        if let Some(focus_req) = child_state.request_focus.take() {
            let old = self.focus_widget;
            let new = match focus_req {
                FocusChange::Focus(child) => Some(child),
                FocusChange::Resign => None,
                _ => None, // TODO: Implement focus chain
            };
            // Only send RouteFocusChanged in case there's actual change
            if old != new {
                let event = LifeCycle::Internal(InternalLifeCycle::RouteFocusChanged { old, new });
                self.lifecycle(&event, false);
                self.focus_widget = new;
            }
        }

        if let Some(cursor) = &child_state.cursor {
            self.handle.as_mut().unwrap().set_cursor(&cursor);
        } else if matches!(
            event,
            Event::MouseMove(..) | Event::Internal(InternalEvent::MouseLeave)
        ) {
            self.handle.as_mut().unwrap().set_cursor(&Cursor::Arrow);
        }

        let mut needs_update = self.root().needs_update();
        while needs_update {
            needs_update = self.root().needs_update();

            let mut context_state = ContextState {
                ext_host: &self.ext_event_host,
                window_id: WindowId::illigal(),
                window: self.handle.as_ref().unwrap(),
                text: self.handle.as_ref().unwrap().text(),
                focus_widget: self.focus_widget,
            };
            let mut cx = Ui::new(
                &mut self.children,
                &mut context_state,
                &mut self.child_counter,
            );
            (self.app)(&mut cx);
        }

        self.post_event_processing(&mut child_state, false);

        is_handled
    }

    pub(crate) fn lifecycle(&mut self, event: &LifeCycle, process_commands: bool) {
        let mut widget_state = ChildState::new(self.root().state.id, Some(self.size));
        let mut context_state = ContextState {
            ext_host: &self.ext_event_host,
            window_id: WindowId::illigal(),
            window: self.handle.as_ref().unwrap(),
            text: self.handle.as_ref().unwrap().text(),
            focus_widget: self.focus_widget,
        };
        let mut ctx = LifeCycleCtx {
            state: &mut context_state,
            child_state: &mut widget_state,
        };
        self.children.renders[0].lifecycle(&mut ctx, event);
        self.post_event_processing(&mut widget_state, process_commands);
    }

    fn post_event_processing(&mut self, widget_state: &mut ChildState, process_commands: bool) {
        // If we need a new paint pass, make sure druid-shell knows it.
        if self.root().state.request_anim {
            self.handle.as_ref().unwrap().request_anim_frame();
        }
        self.invalid.union_with(&widget_state.invalid);

        if widget_state.needs_layout {
            self.layout();
            self.invalid.add_rect(self.size.to_rect());
            widget_state.needs_layout = false;
        }

        if !self.invalid.is_empty() {
            self.handle
                .as_ref()
                .unwrap()
                .invalidate_rect(self.invalid.bounding_box())
        }
        self.invalid.clear();

        // If there are any commands and they should be processed
        if process_commands {
            // Ask the handler to call us back on idle
            // so we can process them in a new event/update pass.
            if let Some(mut handle) = self.handle.as_ref().unwrap().get_idle_handle() {
                handle.schedule_idle(RUN_COMMANDS_TOKEN);
            } else {
                tracing::error!("failed to get idle handle");
            }
        }
    }
}

impl AppHandle {
    pub fn new(app: impl FnMut(&mut Ui) + 'static, shell_app: shell::Application) -> Self {
        let inner_handle = AppHandleInner {
            app: Box::new(app),
            shell_app,
            handle: None,
            ext_event_host: ExtEventHost::new(),
            child_counter: ChildCounter::new(),
            children: Children::new(),
            last_mouse_pos: None,
            focus_widget: None,
            env: crate::env::Env::default(),
            invalid: Region::EMPTY,
            size: Size::ZERO,
        };
        AppHandle {
            inner: Rc::new(RefCell::new(inner_handle)),
        }
    }

    pub fn initialize(&self) {
        self.inner.borrow_mut().initialize()
    }

    pub fn connect(&self, handle: &shell::WindowHandle) {
        self.inner.borrow_mut().connect(handle)
    }

    pub fn prepare_paint(&self) {
        self.inner.borrow_mut().prepare_paint()
    }

    pub fn paint(&self, piet: &mut Piet, invalid: &Region) {
        self.inner.borrow_mut().paint(piet, invalid)
    }

    pub fn do_window_event(&mut self, event: Event, window_id: WindowId) -> Handled {
        self.inner.borrow_mut().event(event)
    }
}
