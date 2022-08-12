# Notes
* Interface should be requested from the created component via get_ method, to allow delegation
* Interface (trait) definition should be placed in a separate module
* (?) How to shape composed interfaces - separate struct with get's? Which returns other structs or trait objects refs?
* Everything in most should be declared as pub(super) 