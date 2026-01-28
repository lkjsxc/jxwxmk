# Security

This section defines the security model and abuse resistance for the single-server game.

The baseline goal is: **no client can crash the server, corrupt world state, or bypass rules** via malformed input.

## Contents

- [Threat model](threat_model.md)
- [Session model](session_model.md)
- [Rate limits + abuse controls](rate_limits.md)
- [Input validation rules](input_validation.md)
