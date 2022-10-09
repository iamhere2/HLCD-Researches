# Notes
* Interface should be requested from the created component explicitly, to allow delegation
* To get interfaces - either own-provided, or delegated (children-provided) we use an additional 
  {X}Provider trait for each interface
* Interface (trait) definition should be placed in a separate module
* Interface references looks as `Rc<RefCell<dyn {InterfaceTrait}>>`
* To deal with borrow checker, we use `Rc<RefCell<_>>` for each intercomponent reference 
* Everything in most should be declared as pub(super)
* Consumed interfaces are passed into `new()` constructors