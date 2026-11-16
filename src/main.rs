mod idle;
mod inhibit;
mod state;

use std::os::fd::OwnedFd;

use cosmic::app::{Core, Task};
use cosmic::{Application, Element};

const ICON_ON: &str = "io.github.bear.CosmicAppletAwake-on-symbolic";
const ICON_OFF: &str = "io.github.bear.CosmicAppletAwake-off-symbolic";

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<Awake>(())
}

struct Awake {
    core: Core,
    /// Some while sleep is blocked. Dropping the fd releases the block.
    lock: Option<OwnedFd>,
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
}

impl Awake {
    fn set(&mut self, on: bool) {
        if on == self.lock.is_some() {
            return;
        }
        if on {
            match inhibit::acquire() {
                Ok(fd) => self.lock = Some(fd),
                Err(e) => {
                    eprintln!("cosmic-applet-awake: inhibit failed: {e}");
                    return;
                }
            }
            // The lock alone doesn't stop COSMIC's own idle suspend; see idle.rs.
            if let Err(e) = idle::pause() {
                eprintln!("cosmic-applet-awake: failed to pause idle suspend: {e}");
            }
        } else {
            self.lock = None;
            if let Err(e) = idle::resume() {
                eprintln!("cosmic-applet-awake: failed to restore idle suspend: {e}");
            }
        }
        state::save(on);
    }
}

impl Application for Awake {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "io.github.bear.CosmicAppletAwake";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: ()) -> (Self, Task<Message>) {
        let mut app = Awake {
            core,
            lock: None,
        };
        if state::load() {
            app.set(true);
        } else if let Err(e) = idle::resume() {
            // Off at startup (e.g. after a reboot): undo a pause left behind
            // by a run that ended while on.
            eprintln!("cosmic-applet-awake: failed to restore idle suspend: {e}");
        }
        (app, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Toggle => self.set(self.lock.is_none()),
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let icon = if self.lock.is_some() { ICON_ON } else { ICON_OFF };
        self.core
            .applet
            .icon_button(icon)
            .on_press(Message::Toggle)
            .into()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}
