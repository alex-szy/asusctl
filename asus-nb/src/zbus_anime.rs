//! # DBus interface proxy for: `org.asuslinux.Daemon`
//!
//! This code was generated by `zbus-xmlgen` `1.0.0` from DBus introspection data.
//! Source: `Interface '/org/asuslinux/Anime' from service 'org.asuslinux.Daemon' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://zeenix.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use zbus::{dbus_proxy, Connection, Result};

use crate::anime_matrix::{AniMeMatrix, AniMePacketType, ANIME_PANE1_PREFIX, ANIME_PANE2_PREFIX};

#[dbus_proxy(
    interface = "org.asuslinux.Daemon",
    default_path = "/org/asuslinux/Anime"
)]
trait Daemon {
    /// SetAnime method
    fn set_anime(&self, input: &[&[u8]]) -> zbus::Result<()>;

    /// SetBootOnOff method
    fn set_boot_on_off(&self, status: bool) -> zbus::Result<()>;

    /// SetOnOff method
    fn set_on_off(&self, status: bool) -> zbus::Result<()>;
}

pub struct AnimeProxy<'a>(DaemonProxy<'a>);

impl<'a> AnimeProxy<'a> {
    #[inline]
    pub fn new(conn: &Connection) -> Result<Self> {
        Ok(AnimeProxy(DaemonProxy::new(&conn)?))
    }

    pub fn proxy(&self) -> &DaemonProxy<'a> {
        &self.0
    }

    #[inline]
    pub fn set_brightness(&self, led_brightness: u8) -> Result<()> {
        let mut anime_matrix = AniMeMatrix::new();

        anime_matrix.fill_with(led_brightness);

        let mut image = AniMePacketType::from(anime_matrix);
        image[0][..7].copy_from_slice(&ANIME_PANE1_PREFIX);
        image[1][..7].copy_from_slice(&ANIME_PANE2_PREFIX);

        self.0.set_anime(&[&image[0], &image[1]])
    }

    #[inline]
    pub fn toggle_boot_on(&self, on: bool) -> Result<()> {
        self.0.set_boot_on_off(on)
    }

    #[inline]
    pub fn toggle_on(&self, on: bool) -> Result<()> {
        self.0.set_on_off(on)
    }
}
