//! Users

use std::borrow::Cow;

use types::{DateTime, WithheldScope};

/// Represents a user on Twitter.
///
/// # Reference
///
/// 1. [Users — Twitter Developers](https://dev.twitter.com/overview/api/users)
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct User<'a> {
    /// Indicates that the user has an account with “contributor mode” enabled,
    /// allowing for Tweets issued by the user to be co-authored
    /// by another account. Rarely `true`.
    pub contributors_enabled: bool,

    /// The UTC datetime that the user account was created on Twitter.
    #[serde(deserialize_with = "::util::deserialize_datetime")]
    pub created_at: DateTime,

    /// When `true`, indicates that the user has not altered
    /// the theme or background of their user profile.
    pub default_profile: bool,

    /// When `true`, indicates that the user has not uploaded their own avatar
    /// and a default egg avatar is used instead.
    pub default_profile_image: bool,

    /// The user-defined UTF-8 string describing their account.
    #[serde(borrow)]
    #[serde(default)]
    #[serde(deserialize_with = "::util::deserialize_opt_cow_str")]
    pub description: Option<Cow<'a, str>>,

    // pub entities: Entities, // does not appear in stream messages

    /// The number of tweets this user has favorited in the account’s lifetime.
    /// British spelling used in the field name for historical reasons.
    pub favourites_count: u64,

    /// *Perspectival*. When `true`, indicates that the authenticating user has
    /// issued a follow request to this protected user account.
    pub follow_request_sent: Option<bool>,

    // pub following: Option<bool>, // deprecated

    /// The number of followers this account currently has. Under certain
    /// conditions of duress, this field will temporarily indicate `0`.
    pub followers_count: u64,

    /// The number of users this account is following (AKA their “followings”).
    /// Under certain conditions of duress, this field will temporarily
    /// indicate `0`.
    pub friends_count: u64,

    /// When `true`, indicates that the user has enabled the possibility of
    /// geotagging their Tweets. This field must be `true` for the current user
    /// to attach geographic data when using [POST statuses / update][1].
    ///
    /// [1]: https://dev.twitter.com/rest/reference/post/statuses/update
    pub geo_enabled: bool,

    // does not appear in stream message
    // pub has_extended_profile: Option<bool>,

    /// The integer representation of the unique identifier for this User.
    pub id: UserId,

    // pub id_str: String,

    /// When `true`, indicates that the user is a participant in Twitter’s
    /// [translator community][1].
    ///
    /// [1]: http://translate.twttr.com/
    pub is_translator: bool,

    /// The [BCP 47][1] code for the user’s self-declared
    /// user interface language. May or may not have anything to do with
    /// the content of their Tweets.
    ///
    /// [1]: http://tools.ietf.org/html/bcp47
    #[serde(borrow)]
    pub lang: Cow<'a, str>,

    /// The number of public lists that this user is a member of.
    pub listed_count: u64,

    /// The user-defined location for this account’s profile.
    /// Not necessarily a location nor parseable. This field will occasionally
    /// be fuzzily interpreted by the Search service.
    #[serde(borrow)]
    #[serde(default)]
    #[serde(deserialize_with = "::util::deserialize_opt_cow_str")]
    pub location: Option<Cow<'a, str>>,

    /// The name of the user, as they’ve defined it. Not necessarily a person’s
    /// name. Typically capped at 20 characters, but subject to change.
    #[serde(borrow)]
    pub name: Cow<'a, str>,

    // pub notifications: Option<bool>, // deprecated

    /// The hexadecimal color chosen by the user for their background.
    #[serde(borrow)]
    pub profile_background_color: Cow<'a, str>,

    /// A HTTP-based URL pointing to the background image
    /// the user has uploaded for their profile.
    #[serde(borrow)]
    pub profile_background_image_url: Cow<'a, str>,

    /// A HTTPS-based URL pointing to the background image
    /// the user has uploaded for their profile.
    #[serde(borrow)]
    pub profile_background_image_url_https: Cow<'a, str>,

    /// When `true`, indicates that the user’s `profile_background_image_url`
    /// should be tiled when displayed.
    pub profile_background_tile: bool,

    /// The HTTPS-based URL pointing to the standard web representation of
    /// the user’s uploaded profile banner.
    /// By adding a final path element of the URL,
    /// you can obtain different image sizes optimized for specific displays.
    ///
    /// In the future, an API method will be provided to serve these URLs
    /// so that you need not modify the original URL.
    ///
    /// For size variations, please see [User Profile Images and Banners][1].
    ///
    /// [1]: https://dev.twitter.com/basics/user-profile-images-and-banners
    #[serde(borrow)]
    #[serde(default)]
    #[serde(deserialize_with = "::util::deserialize_opt_cow_str")]
    pub profile_banner_url: Option<Cow<'a, str>>,

    /// A HTTP-based URL pointing to the user’s avatar image.
    /// See [User Profile Images and Banners][1].
    ///
    /// [1]: https://dev.twitter.com/basics/user-profile-images-and-banners
    #[serde(borrow)]
    pub profile_image_url: Cow<'a, str>,

    /// A HTTPS-based URL pointing to the user’s avatar image.
    #[serde(borrow)]
    pub profile_image_url_https: Cow<'a, str>,

    /// The hexadecimal color the user has chosen
    /// to display links with in their Twitter UI.
    #[serde(borrow)]
    pub profile_link_color: Cow<'a, str>,

    // pub profile_location: Option<_>, // does not appear in stream message

    /// The hexadecimal color the user has chosen
    /// to display sidebar borders with in their Twitter UI.
    #[serde(borrow)]
    pub profile_sidebar_border_color: Cow<'a, str>,

    /// The hexadecimal color the user has chosen
    /// to display sidebar backgrounds with in their Twitter UI.
    #[serde(borrow)]
    pub profile_sidebar_fill_color: Cow<'a, str>,

    /// The hexadecimal color the user has chosen
    /// to display text with in their Twitter UI.
    #[serde(borrow)]
    pub profile_text_color: Cow<'a, str>,

    /// When `true`, indicates the user wants their uploaded background image
    /// to be used.
    pub profile_use_background_image: bool,

    /// When `true`, indicates that this user has chosen to protect
    /// their Tweets. See [About Public and Protected Tweets][1].
    ///
    /// [1]: https://support.twitter.com/articles/14016-about-public-and-protected-tweets
    pub protected: bool,

    /// The screen name, handle, or alias that this user identifies themselves
    /// with.
    ///
    /// `screen_name`s are unique but subject to change.
    /// Use `id` as a user identifier whenever possible.
    ///
    /// Typically a maximum of 15 characters long,
    /// but some historical accounts may exist with longer names.
    #[serde(borrow)]
    pub screen_name: Cow<'a, str>,

    // pub show_all_inline_media: bool, // removed

    // pub status: Option<Box<Tweet>>, // does not appear in stream messages

    /// The number of tweets (including retweets) issued by the user.
    pub statuses_count: u64,

    /// A string describing the Time Zone this user declares themselves within.
    #[serde(borrow)]
    #[serde(default)]
    #[serde(deserialize_with = "::util::deserialize_opt_cow_str")]
    pub time_zone: Option<Cow<'a, str>>,

    /// A URL provided by the user in association with their profile.
    #[serde(borrow)]
    #[serde(default)]
    #[serde(deserialize_with = "::util::deserialize_opt_cow_str")]
    pub url: Option<Cow<'a, str>>,

    /// The offset from GMT/UTC in seconds.
    pub utc_offset: Option<i64>,

    /// When `true`, indicates that the user has a verified account.
    /// See [Verified Accounts][1].
    ///
    /// [1]: https://support.twitter.com/articles/119135-faqs-about-verified-accounts
    pub verified: bool,

    /// When present, indicates a textual representation of
    /// the two-letter country codes this user is withheld from.
    #[serde(borrow)]
    #[serde(default)]
    #[serde(deserialize_with = "::util::deserialize_opt_cow_str")]
    pub withheld_in_countries: Option<Cow<'a, str>>,

    /// When present, indicates whether the content being withheld is
    /// the `Status` or a `User`.
    #[serde(borrow)]
    pub withheld_scope: Option<WithheldScope<'a>>,
}

/// Numerical ID of a user.
pub type UserId = u64;
