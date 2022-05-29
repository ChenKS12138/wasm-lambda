use std::ops::Deref;

use serde::de::DeserializeOwned;

use crate::value;

#[derive(Debug, Clone)]
pub struct Params(value::Params);

impl From<(&value::TriggerEvent, &value::Params)> for Params {
    fn from(args: (&value::TriggerEvent, &value::Params)) -> Self {
        Self(args.1.clone())
    }
}

impl Deref for Params {
    type Target = value::Params;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Query(qstring::QString);

impl From<(&value::TriggerEvent, &value::Params)> for Query {
    fn from(args: (&value::TriggerEvent, &value::Params)) -> Self {
        let request = match args.0 {
            value::TriggerEvent::EventHttpRequest(request) => request,
            value::TriggerEvent::EventInternalModuleCall(_, request) => request,
        };
        match request.path.find("?") {
            None => Self(qstring::QString::new::<String, String>(vec![])),
            Some(index) => Self(qstring::QString::from(&request.path[index..])),
        }
    }
}

impl Deref for Query {
    type Target = qstring::QString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct FormData(form_data::FormData<String>);

impl From<(&value::TriggerEvent, &value::Params)> for FormData {
    fn from(_args: (&value::TriggerEvent, &value::Params)) -> Self {
        unimplemented!("FormData is not implemented yet")
    }
}

impl Deref for FormData {
    type Target = form_data::FormData<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct FormUrlEncoded(qstring::QString);

impl From<(&value::TriggerEvent, &value::Params)> for FormUrlEncoded {
    fn from(args: (&value::TriggerEvent, &value::Params)) -> Self {
        let request = match args.0 {
            value::TriggerEvent::EventHttpRequest(request) => request,
            value::TriggerEvent::EventInternalModuleCall(_, request) => request,
        };
        Self(qstring::QString::from(&request.path[..]))
    }
}

impl Deref for FormUrlEncoded {
    type Target = qstring::QString;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Json<T>(T);

impl<T: DeserializeOwned> From<(&value::TriggerEvent, &value::Params)> for Json<T> {
    fn from(args: (&value::TriggerEvent, &value::Params)) -> Self {
        let request = match args.0 {
            value::TriggerEvent::EventHttpRequest(request) => request,
            value::TriggerEvent::EventInternalModuleCall(_, request) => request,
        };
        let body = request
            .body
            .as_ref()
            .and_then(|v| Some(v.clone()))
            .unwrap_or(Vec::new());
        let result = serde_json::from_slice(&body).unwrap();
        Self(result)
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Body(Vec<u8>);

impl From<(&value::TriggerEvent, &value::Params)> for Body {
    fn from(args: (&value::TriggerEvent, &value::Params)) -> Self {
        let request = match args.0 {
            value::TriggerEvent::EventHttpRequest(request) => request,
            value::TriggerEvent::EventInternalModuleCall(_, request) => request,
        };
        Self(
            request
                .body
                .as_ref()
                .and_then(|v| Some(v.clone()))
                .unwrap_or(Vec::new()),
        )
    }
}

impl Deref for Body {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct RequestSource(Option<String>);

impl From<(&value::TriggerEvent, &value::Params)> for RequestSource {
    fn from(args: (&value::TriggerEvent, &value::Params)) -> Self {
        Self(match args.0 {
            value::TriggerEvent::EventHttpRequest(_) => None,
            value::TriggerEvent::EventInternalModuleCall(source, _) => Some(source.clone()),
        })
    }
}

impl Deref for RequestSource {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct TriggerEvent(value::TriggerEvent);

impl From<(&value::TriggerEvent, &value::Params)> for TriggerEvent {
    fn from(args: (&value::TriggerEvent, &value::Params)) -> Self {
        Self(args.0.clone())
    }
}

impl Deref for TriggerEvent {
    type Target = value::TriggerEvent;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Headers(std::collections::HashMap<String, String>);

impl From<(&value::TriggerEvent, &value::Params)> for Headers {
    fn from(args: (&value::TriggerEvent, &value::Params)) -> Self {
        let request = match args.0 {
            value::TriggerEvent::EventHttpRequest(request) => request,
            value::TriggerEvent::EventInternalModuleCall(_, request) => request,
        };
        Self(
            request
                .headers
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        )
    }
}

impl Deref for Headers {
    type Target = std::collections::HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
