//! Commonly used items.

pub use crate::{
    dispatching::{
        default_updates_stream,
        dialogue::{
            exit, next, DialogueDispatcher, DialogueStage, DialogueWithCx, GetChatId, Transition,
            TransitionIn, TransitionOut,
        },
        Dispatcher, DispatcherHandlerRx, DispatcherHandlerRxExt, UpdateWithCx,
    },
    error_handlers::{LoggingErrorHandler, OnError},
    requests::{respond, Request, ResponseResult},
    types::{Message, Update, UpdateKind},
    Bot, RequestError,
};

#[cfg(feature = "frunk")]
// FIXME(waffle): use `docsrs` here when issue with combine is resolved <https://github.com/teloxide/teloxide/pull/305#issuecomment-716172103>
#[cfg_attr(all(teloxide_docsrs, feature = "nightly"), doc(cfg(feature = "frunk")))]
pub use crate::utils::UpState;

pub use tokio::sync::mpsc::UnboundedReceiver;

pub use futures::StreamExt;
