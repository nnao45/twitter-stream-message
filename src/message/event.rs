use std::fmt;

use serde::de::{
    Deserialize,
    Deserializer,
    Error,
    IgnoredAny,
    MapAccess,
    Visitor,
};

use {List, Tweet, User};
use types::{DateTime, JsonValue};
use util;

/// Represents notifications about non-Tweet events are also sent over a stream.
///
/// # Reference
///
/// 1. [Streaming message types — Twitter Developers][1]
///
/// [1]: https://dev.twitter.com/streaming/overview/messages-types#Events_event
#[derive(Clone, Debug, PartialEq)]
pub struct Event<'a> {
    pub created_at: DateTime,

    /// An object which indicates the name of the event and contains
    /// an optional object which represents the target of the event.
    pub event: EventKind<'a>,

    pub target: User<'a>,

    pub source: User<'a>,
}

macro_rules! impl_event {
    (
        $(#[$attr:meta])*
        pub enum $T:ident<$lifetime:tt> {
            $(
                $(#[$c_attr:meta])*
                $Container:ident($c_tag:expr, $Content:ty)
            ),*;
            $(
                $(#[$l_attr:meta])*
                $Label:ident($l_tag:expr)
            ),*;
            $(#[$cu_attr:meta])*
            $Custom:ident(_, _),
        }
    ) => {
        $(#[$attr])*
        pub enum $T<$lifetime> {
            $(
                $(#[$c_attr])*
                $Container($Content),
            )*
            $(
                $(#[$l_attr])*
                $Label,
            )*
            $(#[$cu_attr])*
            $Custom(::std::borrow::Cow<$lifetime, str>, Option<JsonValue>),
        }

        impl<'de: 'a, 'a> Deserialize<'de> for Event<'a> {
            fn deserialize<D: Deserializer<'de>>(d: D)
                -> Result<Self, D::Error>
            {
                d.deserialize_map(EventVisitor)
            }
        }

        struct EventVisitor;

        impl<'a> Visitor<'a> for EventVisitor {
            type Value = Event<'a>;

            fn visit_map<A: MapAccess<'a>>(self, mut a: A)
                -> Result<Event<'a>, A::Error>
            {
                use util::CowStr;

                #[derive(Default)]
                struct EventBuffer<'a> {
                    created_at: Option<DateTime>,
                    event: Option<EventKind<'a>>,
                    target: Option<User<'a>>,
                    source: Option<User<'a>>,
                }

                let mut event = EventBuffer::default();
                let mut event_kind: Option<CowStr> = None;
                let mut target_obj: Option<JsonValue> = None;

                while let Some(k) = a.next_key::<CowStr>()? {
                    match &*k {
                        "created_at" => {
                            let val = a.next_value::<CowStr>()?;
                            let dt = util::parse_datetime(&*val)
                                .map_err(A::Error::custom)?;
                            event.created_at = Some(dt);
                        },
                        "event" => {
                            let e = a.next_value::<CowStr>()?;
                            event.event = if let Some(t) = target_obj.take() {
                                match &*e {
                                    $($c_tag => {
                                        let c = <$Content>::deserialize(t)
                                            .map_err(A::Error::custom)?;
                                        $T::$Container(c)
                                    },)*
                                    $($l_tag => $T::$Label,)*
                                    _ => $T::$Custom(e.0, Some(t)),
                                }.into()
                            } else {
                                match &*e {
                                    $($c_tag)|* => {
                                        event_kind = Some(e);
                                        None
                                    },
                                    $($l_tag => Some($T::$Label),)*
                                    _ => Some($T::Custom(e.0, None)),
                                }
                            };
                        },
                        "target" => event.target = Some(a.next_value()?),
                        "source" => event.source = Some(a.next_value()?),
                        "target_object" => if let Some(e) = event_kind.take() {
                            event.event = match &*e {
                                $($c_tag => $T::$Container(a.next_value()?),)*
                                $($l_tag => { a.next_value::<IgnoredAny>()?; $T::$Label },)*
                                _ => $T::$Custom(e.0, a.next_value()?),
                            }.into();
                        } else if event.event.is_none() {
                            target_obj = Some(a.next_value()?);
                        } else {
                            a.next_value::<IgnoredAny>()?;
                        },
                        _ => { a.next_value::<IgnoredAny>()?; },
                    }

                    if let EventBuffer {
                        created_at: Some(created_at),
                        event: Some(event),
                        target: Some(target),
                        source: Some(source),
                    } = event
                    {
                        while a.next_entry::<IgnoredAny, IgnoredAny>()?
                            .is_some() {}
                        return Ok(Event { created_at, event, target, source });
                    }
                }

                Err(A::Error::missing_field(match event {
                    EventBuffer { created_at: None, .. } => "created_at",
                    EventBuffer { target: None, .. } => "target",
                    EventBuffer { source: None, .. } => "source",
                    EventBuffer { event: None, .. } => if target_obj.is_some() {
                        "event"
                    } else {
                        "target_object"
                    },
                    EventBuffer {
                        created_at: Some(_),
                        target: Some(_),
                        source: Some(_),
                        event: Some(_),
                    } => unreachable!(),
                }))
            }

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a map")
            }
        }
    };
}

impl_event! {
    /// An object which indicates the name of an event. It may contain an
    /// object called "target object" which represents the target of the event.
    ///
    /// The meaning of `target` and `source` field of an `Event` will
    /// be different based on the name of the event, as described below.
    ///
    /// | Description                         | Event Name             | `source`           | `target`       |
    /// | ----------------------------------- | ---------------------- | ------------------ | -------------- |
    /// | User deauthorizes stream            | `AccessRevoked`        | Deauthorizing user | App owner      |
    /// | User blocks someone                 | `Block`                | Current user       | Blocked user   |
    /// | User removes a block                | `Unblock`              | Current user       | Unblocked user |
    /// | User favorites a Tweet              | `Favorite`             | Current user       | Tweet author   |
    /// | User's Tweet is favorited           | `Favorite`             | Favoriting user    | Current user   |
    /// | User unfavorites a Tweet            | `Unfavorite`           | Current user       | Tweet author   |
    /// | User's Tweet is unfavorited         | `Unfavorite`           | Unfavoriting user  | Current user   |
    /// | User follows someone                | `Follow`               | Current user       | Followed user  |
    /// | User is followed                    | `Follow`               | Following user     | Current user   |
    /// | User unfollows someone              | `Unfollow`             | Current user       | Followed user  |
    /// | User creates a list                 | `ListCreated`          | Current user       | Current user   |
    /// | User deletes a list                 | `ListDestroyed`        | Current user       | Current user   |
    /// | User edits a list                   | `ListUpdated`          | Current user       | Current user   |
    /// | User adds someone to a list         | `ListMemberAdded`      | Current user       | Added user     |
    /// | User is added to a list             | `ListMemberAdded`      | Adding user        | Current user   |
    /// | User removes someone from a list    | `ListMemberRemoved`    | Current user       | Removed user   |
    /// | User is removed from a list         | `ListMemberRemoved`    | Removing user      | Current user   |
    /// | User subscribes to a list           | `ListUserSubscribed`   | Current user       | List owner     |
    /// | User's list is subscribed to        | `ListUserSubscribed`   | Subscribing user   | Current user   |
    /// | User unsubscribes from a list       | `ListUserUnsubscribed` | Current user       | List owner     |
    /// | User's list is unsubscribed from    | `ListUserUnsubscribed` | Unsubscribing user | Current user   |
    /// | User's Tweet is quoted              | `QuotedTweet`          | quoting User       | Current User   |
    /// | User updates their profile          | `UserUpdate`           | Current user       | Current user   |
    /// | User updates their protected status | `UserUpdate`           | Current user       | Current user   |
    #[derive(Clone, Debug, PartialEq)]
    pub enum EventKind<'a> {
        Favorite("favorite", Box<Tweet<'a>>),
        Unfavorite("unfavorite", Box<Tweet<'a>>),
        ListCreated("list_created", Box<List<'a>>),
        ListDestroyed("list_destroyed", Box<List<'a>>),
        ListUpdated("list_updated", Box<List<'a>>),
        ListMemberAdded("list_member_added", Box<List<'a>>),
        ListMemberRemoved("list_member_removed", Box<List<'a>>),
        ListUserSubscribed("list_user_subscribed", Box<List<'a>>),
        ListUserUnsubscribed("list_user_unsubscribed", Box<List<'a>>),
        QuotedTweet("quoted_tweet", Box<Tweet<'a>>);
        AccessRevoked("access_revoked"),
        Block("block"),
        Unblock("unblock"),
        Follow("follow"),
        Unfollow("unfollow"),
        UserUpdate("user_update");
        /// An event this library does not know. The first value is raw event name
        /// and the second is the target object.
        Custom(_, _),
    }
}
