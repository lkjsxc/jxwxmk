# Game Engine

Fixed tick loop, event queue, and broadcasting logic.

## Contents

- `mod.rs`: engine struct and actor startup.
- `messages.rs`: actix message types.
- `helpers.rs`: persistence and session send helpers.
- `tick.rs`: tick loop and input processing.
- `broadcast.rs`: chunk streaming.
- `streaming.rs`: chunk/entity serialization.
- `handlers_session.rs`: join/leave/revoke handlers.
- `handlers_input.rs`: input/action handlers.
