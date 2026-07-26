// use cosmic::iced::platform_specific::shell::commands::popup::{destroy_popup, get_popup};
use cosmic::iced::Subscription;
use cosmic::iced::window::Id;
use cosmic::prelude::*;
use cosmic::widget::{self, autosize};
use std::sync::LazyLock;
use std::time::Duration;
use std::{fs, thread};

#[derive(Default)]
pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    button_text: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    // TogglePopup,
    UpdateCpuPct,
    PopupClosed(Id),
}

const AUTOSIZE_MAIN_ID: LazyLock<cosmic::widget::Id> =
    LazyLock::new(|| cosmic::widget::Id::new("text-widget-id"));

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.github.cosmic_ext.SystemMonitor";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let app = AppModel {
            core,
            button_text: "0.0%   ".to_string(),
            ..Default::default()
        };

        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let text_widget = self.core.applet.text(&self.button_text);

        autosize::autosize(text_widget, AUTOSIZE_MAIN_ID.clone()).into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        widget::column![].into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        cosmic::iced::time::every(Duration::from_secs(3)).map(|_| Message::UpdateCpuPct)
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::UpdateCpuPct => {
                self.button_text = get_cpu_pct();
            }
            // Message::TogglePopup => {
            //     return if let Some(p) = self.popup.take() {
            //         destroy_popup(p)
            //     } else {
            //         let new_id = Id::unique();
            //         self.popup.replace(new_id);
            //         let mut popup_settings = self.core.applet.get_popup_settings(
            //             self.core.main_window_id().unwrap(),
            //             new_id,
            //             None,
            //             None,
            //             None,
            //         );
            //         popup_settings.positioner.size_limits = Limits::NONE
            //             .max_width(372.0)
            //             .min_width(300.0)
            //             .min_height(200.0)
            //             .max_height(1080.0);
            //         get_popup(popup_settings)
            //     };
            // }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

fn sample_cpu() -> (u64, u64) {
    let stat = fs::read_to_string("/proc/stat").unwrap();
    let v: Vec<u64> = stat
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .take(8)
        .map(|s| s.parse().unwrap())
        .collect();
    (v.iter().sum(), v[3] + v[4]) // total, idle + iowait
}

fn get_cpu_pct() -> String {
    let (t0, i0) = sample_cpu();
    thread::sleep(Duration::from_secs(1));
    let (t1, i1) = sample_cpu();
    format!(
        "{:.1}%   ",
        100.0 * (1.0 - (i1 - i0) as f64 / (t1 - t0) as f64)
    )
}
