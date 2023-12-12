use std::fmt::{self, Display, Formatter};
use windows::{
    core::HSTRING,
    Data::Xml::Dom::XmlDocument,
    UI::Notifications::{ToastNotification, ToastNotificationManager},
};

#[derive(Debug)]
pub struct Toast {
    app_id: String,
    text1: String,
    text2: Option<String>,
    duration: Duration,
}

#[derive(Debug)]
pub enum Duration {
    Short,
    Long,
}

impl Toast {
    pub fn new(app_id: &str, text1: &str) -> Toast {
        Toast {
            app_id: app_id.to_string(),
            text1: text1.to_string(),
            text2: None,
            duration: Duration::Long,
        }
    }

    pub fn text2(&mut self, text2: &str) {
        self.text2 = Some(text2.to_string());
    }

    pub fn duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    pub fn notify(&self) {
        let toast_xml = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
        <toast activationType="protocol" duration="{}">
            <visual>
                <binding template="ToastGeneric">
                    <text>{}</text>
                </binding>
            </visual>
        </toast>"#,
            self.duration, self.text1
        );

        let xml = XmlDocument::new().unwrap();
        xml.LoadXml(&HSTRING::from(toast_xml)).unwrap();

        if let Some(text2) = &self.text2 {
            let text = xml.CreateElement(&HSTRING::from("text")).unwrap();
            text.SetInnerText(&HSTRING::from(text2)).unwrap();
            xml.SelectSingleNode(&HSTRING::from("/toast/visual/binding"))
                .unwrap()
                .AppendChild(&text)
                .unwrap();
        }
        let template = ToastNotification::CreateToastNotification(&xml).unwrap();

        let notifier =
            ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(&self.app_id))
                .unwrap();
        notifier.Show(&template).unwrap();
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Duration::Short => write!(f, "short"),
            Duration::Long => write!(f, "long"),
        }
    }
}
