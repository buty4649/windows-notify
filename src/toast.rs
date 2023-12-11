use windows::{
    core::HSTRING,
    Data::Xml::Dom::XmlDocument,
    UI::Notifications::{ToastNotification, ToastNotificationManager},
};

pub struct Toast {
    app_id: String,
    text1: String,
    text2: Option<String>,
}

impl Toast {
    pub fn new(app_id: &str, text1: &str, text2: Option<&str>) -> Toast {
        Toast {
            app_id: app_id.to_string(),
            text1: text1.to_string(),
            text2: text2.map(|s| s.to_string()),
        }
    }

    pub fn notify(&self) {
        let toast_xml = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
        <toast activationType="protocol" launch="{}">
            <visual>
                <binding template="ToastGeneric">
                    <text>{}</text>
                </binding>
            </visual>
        </toast>"#,
            self.app_id, self.text1
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
