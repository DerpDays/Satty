use gtk::prelude::*;
use relm4::{gtk::Align, prelude::*};

pub struct Toast {
    text: String,
    visible: bool,
    next_timer: u64,
    timeout: u64,
}

#[derive(Debug)]
pub enum ToastMessage {
    Show(String),
}

#[derive(Debug)]
pub enum ToastCommand {
    Hide(u64),
}

#[relm4::component(pub)]
impl Component for Toast {
    type Init = u64;
    type Input = ToastMessage;
    type Output = ();
    type CommandOutput = ToastCommand;

    view! {
        root = gtk::Box {
            set_valign: Align::Start,
            set_halign: Align::Center,
            add_css_class: "toast",

            #[watch]
            set_visible: model.visible,

            gtk::Label {
                set_margin_top: 10,
                set_margin_bottom: 10,
                set_margin_start: 16,
                set_margin_end: 16,


                #[watch]
                set_text: &model.text
            }
        },
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            ToastMessage::Show(text) => {
                self.text = text;
                self.visible = true;
                self.next_timer += 1;

                let next_timer = self.next_timer;
                let timeout = self.timeout;
                sender.oneshot_command(async move {
                    tokio::time::sleep(std::time::Duration::from_millis(timeout)).await;
                    ToastCommand::Hide(next_timer)
                });
            }
        }
    }

    fn update_cmd(
        &mut self,
        command: Self::CommandOutput,
        _: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match command {
            ToastCommand::Hide(this_timer) => {
                if self.next_timer == this_timer {
                    self.visible = false;
                }
            }
        }
    }

    fn init(
        timeout: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            text: String::new(),
            visible: false,
            next_timer: 0,
            timeout,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
