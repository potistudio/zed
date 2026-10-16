use copilot::{Copilot, GlobalCopilotAuth};
use copilot_chat::CopilotChat;
use gpui::{App, Context, Entity, IntoElement, Render, Subscription, TaskExt, Window};
use ui::{ConfiguredApiCard, prelude::*};
use workspace::{AppState, Toast, Workspace, notifications::NotificationId};

struct CopilotStatusToast;

pub fn initiate_sign_out(copilot: Entity<Copilot>, window: &mut Window, cx: &mut App) {
    copilot_toast(Some("Signing out of Copilot Edit Predictions…"), window, cx);

    let sign_out_task = copilot.update(cx, |copilot, cx| copilot.sign_out(cx));
    window
        .spawn(cx, async move |cx| match sign_out_task.await {
            Ok(()) => cx.update(|window, cx| {
                copilot_toast(Some("Signed out of Copilot Edit Predictions"), window, cx)
            }),
            Err(err) => cx.update(|window, cx| {
                if let Some(workspace) = Workspace::for_window(window, cx) {
                    workspace.update(cx, |workspace, cx| {
                        workspace.show_error(format!("Error: {err}"), cx);
                    })
                } else {
                    log::error!("{:?}", err);
                }
            }),
        })
        .detach();
}

fn copilot_toast(message: Option<&'static str>, window: &Window, cx: &mut App) {
    const NOTIFICATION_ID: NotificationId = NotificationId::unique::<CopilotStatusToast>();

    let Some(workspace) = Workspace::for_window(window, cx) else {
        return;
    };

    cx.defer(move |cx| {
        workspace.update(cx, |workspace, cx| match message {
            Some(message) => workspace.show_toast(Toast::new(NOTIFICATION_ID, message), cx),
            None => workspace.dismiss_toast(&NOTIFICATION_ID, cx),
        });
    })
}

pub struct ConfigurationView {
    is_authenticated: Box<dyn Fn(&mut App) -> bool + 'static>,
    edit_prediction: bool,
    compact: bool,
    _subscription: Option<Subscription>,
}

pub enum ConfigurationMode {
    Chat,
    EditPrediction,
}

impl ConfigurationView {
    pub fn new(
        is_authenticated: impl Fn(&mut App) -> bool + 'static,
        mode: ConfigurationMode,
        cx: &mut Context<Self>,
    ) -> Self {
        let edit_prediction = matches!(mode, ConfigurationMode::EditPrediction);
        let subscription = match mode {
            ConfigurationMode::EditPrediction => AppState::try_global(cx)
                .and_then(|state| GlobalCopilotAuth::try_get_or_init(state, cx))
                .map(|copilot| cx.observe(&copilot.0, |_, _, cx| cx.notify())),
            ConfigurationMode::Chat => {
                CopilotChat::global(cx).map(|copilot| cx.observe(&copilot, |_, _, cx| cx.notify()))
            }
        };

        Self {
            is_authenticated: Box::new(is_authenticated),
            edit_prediction,
            compact: false,
            _subscription: subscription,
        }
    }

    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let edit_prediction = self.edit_prediction;
        if (self.is_authenticated)(cx) {
            return ConfiguredApiCard::new("copilot-authorized", "Authorized")
                .button_label("Sign Out")
                .on_click(move |_, window, cx| {
                    if edit_prediction {
                        if let Some(auth) = GlobalCopilotAuth::try_global(cx) {
                            initiate_sign_out(auth.0.clone(), window, cx);
                        }
                    } else if let Some(copilot_chat) = CopilotChat::global(cx) {
                        copilot_chat
                            .update(cx, |copilot_chat, cx| copilot_chat.sign_out(cx))
                            .detach_and_log_err(cx);
                    }
                })
                .into_any_element();
        }

        v_flex()
            .when(!self.compact, |this| {
                this.child(Label::new(
                    "Copilot requires an authenticated GitHub account.",
                ))
            })
            .child(
                Label::new("Sign-in is unavailable. Configure an API-key provider instead.")
                    .color(Color::Muted),
            )
            .into_any_element()
    }
}
