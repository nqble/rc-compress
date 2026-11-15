//! Launches handed over by other rc-compress processes while this one is still setting up, kept until it is ready.
//!
//! The single-instance plugin passes another launch's arguments (e.g. a file chosen with Quick Compress) to this
//! process as soon as its event window gets a message, and Windows delivers those while the app is still creating its
//! main window, before setup has started logging and made the queue (`DropQueueState`). Handled then, the files were
//! silently dropped: with the app closed, 1 of 5 files selected together was compressed (2026-11-07). So until
//! `replay` runs at the end of setup, launches are held here, then handled in the order they came.

use std::sync::Mutex;

/// A launch's arguments & working folder, as the plugin passes them
pub type Launch = (Vec<String>, String);

/// `Some` while the app is setting up (the launches held so far), `None` once it is ready
static HELD: Mutex<Option<Vec<Launch>>> = Mutex::new(Some(Vec::new()));

/// Keeps `launch` for later while the app is setting up. Returns whether it was kept; if not, handle it now.
pub fn hold(launch: Launch) -> bool {
    hold_in(&HELD, launch)
}

/// Marks the app ready and hands each held launch, oldest first, to `handle`
pub fn replay(handle: impl FnMut(Launch)) {
    replay_from(&HELD, handle)
}

fn hold_in(held: &Mutex<Option<Vec<Launch>>>, launch: Launch) -> bool {
    let Ok(mut held) = held.lock() else {
        return false;
    };
    let Some(launches) = held.as_mut() else {
        return false;
    };
    launches.push(launch);
    true
}

fn replay_from(held: &Mutex<Option<Vec<Launch>>>, handle: impl FnMut(Launch)) {
    // Taken out first, so a launch arriving while these are handled is handled at once, not held
    let launches = match held.lock() {
        Ok(mut held) => held.take().unwrap_or_default(),
        Err(_) => return,
    };
    launches.into_iter().for_each(handle);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn launch(file: &str) -> Launch {
        (vec!["rc-compress".into(), "--quick-compress".into(), file.into()], String::new())
    }

    #[test]
    fn launches_while_setting_up_are_held_and_replayed_in_order() {
        let held = Mutex::new(Some(Vec::new()));
        assert!(hold_in(&held, launch("a.mp4")));
        assert!(hold_in(&held, launch("b.jpg")));
        let mut handled = Vec::new();
        replay_from(&held, |(args, _)| handled.push(args[2].clone()));
        assert_eq!(handled, ["a.mp4", "b.jpg"]);
    }

    #[test]
    fn launches_once_ready_are_not_held() {
        let held = Mutex::new(Some(Vec::new()));
        replay_from(&held, |_| {});
        assert!(!hold_in(&held, launch("c.png")));
    }

    #[test]
    fn replaying_twice_hands_each_launch_over_once() {
        let held = Mutex::new(Some(vec![launch("a.mp4")]));
        let mut count = 0;
        replay_from(&held, |_| count += 1);
        replay_from(&held, |_| count += 1);
        assert_eq!(count, 1);
    }
}
