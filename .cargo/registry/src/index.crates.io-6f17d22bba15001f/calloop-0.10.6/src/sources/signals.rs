//! Event source for tracking Unix signals
//!
//! Only available on Linux.
//!
//! This allows you to track  and receive Unix signals through the event loop
//! rather than by registering signal handlers. It uses `signalfd` under the hood.
//!
//! The source will take care of masking and unmasking signals for the thread it runs on,
//! but you are responsible for masking them on other threads if you run them. The simplest
//! way to ensure that is to setup the signal event source before spawning any thread, as
//! they'll inherit their parent signal mask.

use std::convert::TryFrom;
use std::os::raw::c_int;

use nix::sys::signal::SigSet;
pub use nix::sys::signal::Signal;
pub use nix::sys::signalfd::siginfo;
use nix::sys::signalfd::{SfdFlags, SignalFd};

use super::generic::Generic;
use crate::{EventSource, Interest, Mode, Poll, PostAction, Readiness, Token, TokenFactory};

/// An event generated by the signal event source
#[derive(Copy, Clone, Debug)]
pub struct Event {
    info: siginfo,
}

impl Event {
    /// Retrieve the signal number that was receive
    pub fn signal(&self) -> Signal {
        Signal::try_from(self.info.ssi_signo as c_int).unwrap()
    }

    /// Access the full `siginfo_t` associated with this signal event
    pub fn full_info(&self) -> siginfo {
        self.info
    }
}

/// An event source for receiving Unix signals
#[derive(Debug)]
pub struct Signals {
    sfd: Generic<SignalFd>,
    mask: SigSet,
}

impl Signals {
    /// Create a new signal event source listening on the specified list of signals
    pub fn new(signals: &[Signal]) -> crate::Result<Signals> {
        let mut mask = SigSet::empty();
        for &s in signals {
            mask.add(s);
        }

        // Mask the signals for this thread
        mask.thread_block()?;
        // Create the SignalFd
        let sfd = SignalFd::with_flags(&mask, SfdFlags::SFD_NONBLOCK | SfdFlags::SFD_CLOEXEC)?;

        Ok(Signals {
            sfd: Generic::new(sfd, Interest::READ, Mode::Level),
            mask,
        })
    }

    /// Add a list of signals to the signals source
    ///
    /// If this function returns an error, the signal mask of the thread may
    /// have still been changed.
    pub fn add_signals(&mut self, signals: &[Signal]) -> crate::Result<()> {
        for &s in signals {
            self.mask.add(s);
        }
        self.mask.thread_block()?;
        self.sfd.file.set_mask(&self.mask)?;
        Ok(())
    }

    /// Remove a list of signals from the signals source
    ///
    /// If this function returns an error, the signal mask of the thread may
    /// have still been changed.
    pub fn remove_signals(&mut self, signals: &[Signal]) -> crate::Result<()> {
        let mut removed = SigSet::empty();
        for &s in signals {
            self.mask.remove(s);
            removed.add(s);
        }
        removed.thread_unblock()?;
        self.sfd.file.set_mask(&self.mask)?;
        Ok(())
    }

    /// Replace the list of signals of the source
    ///
    /// If this function returns an error, the signal mask of the thread may
    /// have still been changed.
    pub fn set_signals(&mut self, signals: &[Signal]) -> crate::Result<()> {
        let mut new_mask = SigSet::empty();
        for &s in signals {
            new_mask.add(s);
        }

        self.mask.thread_unblock()?;
        new_mask.thread_block()?;
        self.sfd.file.set_mask(&new_mask)?;
        self.mask = new_mask;

        Ok(())
    }
}

impl Drop for Signals {
    fn drop(&mut self) {
        // we cannot handle error here
        if let Err(e) = self.mask.thread_unblock() {
            log::warn!("[calloop] Failed to unmask signals: {:?}", e);
        }
    }
}

impl EventSource for Signals {
    type Event = Event;
    type Metadata = ();
    type Ret = ();
    type Error = SignalError;

    fn process_events<C>(
        &mut self,
        readiness: Readiness,
        token: Token,
        mut callback: C,
    ) -> Result<PostAction, Self::Error>
    where
        C: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        self.sfd
            .process_events(readiness, token, |_, sfd| {
                loop {
                    match sfd.read_signal() {
                        Ok(Some(info)) => callback(Event { info }, &mut ()),
                        Ok(None) => break,
                        Err(e) => {
                            log::warn!("[callop] Error reading from signalfd: {}", e);
                            return Err(e.into());
                        }
                    }
                }
                Ok(PostAction::Continue)
            })
            .map_err(|e| SignalError(e.into()))
    }

    fn register(&mut self, poll: &mut Poll, token_factory: &mut TokenFactory) -> crate::Result<()> {
        self.sfd.register(poll, token_factory)
    }

    fn reregister(
        &mut self,
        poll: &mut Poll,
        token_factory: &mut TokenFactory,
    ) -> crate::Result<()> {
        self.sfd.reregister(poll, token_factory)
    }

    fn unregister(&mut self, poll: &mut Poll) -> crate::Result<()> {
        self.sfd.unregister(poll)
    }
}

/// An error arising from processing events for a process signal.
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct SignalError(Box<dyn std::error::Error + Sync + Send>);
