use robotics_lib::event::events::Event;

/// Converts an event to a recognizable key ignoring the event properties
pub(crate) fn event_key(event: &Event) -> &str {
    match event {
        Event::Ready => { "ready" }
        Event::Terminated => { "terminated" }
        Event::TimeChanged(_) => { "time_changed" }
        Event::DayChanged(_) => { "day_changed" }
        Event::EnergyRecharged(_) => { "energy_recharged" }
        Event::EnergyConsumed(_) => { "energy_consumed" }
        Event::Moved(_, _) => { "moved" }
        Event::TileContentUpdated(_, _) => { "tile_content_updated" }
        Event::AddedToBackpack(_, _) => { "added_to_backpack" }
        Event::RemovedFromBackpack(_, _) => { "removed_from_backpack" }
    }
}