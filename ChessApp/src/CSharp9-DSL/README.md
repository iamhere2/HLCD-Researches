# Idea

* This example differs from CSharp8 not only by the used C# language version,
  but also by the usage of special DSL-attributes for components markup.
* All the classes in application assembly should be marked with a special CLR-attributes,
  to classify them strictly as data values / components / interfaces
  and to place them explicitly into the component hierarchy.
* A special Roslyn-analyzer and code generator should be used
  to create easy-to-read/analyze runtime metadata model
  and to perform all the necessary checks about the model conformity
  and all the class interactions.
* [Goal!] The resulting metadata model should be easily used to calculate some useful metrics
  about the component hierarchy and the design quality.
* [Goal!] The resulting metadata model should be enough to generate automatically
  a decent series of UML component diagrams (probably, with PlantUML).
* (?) Code generation and "component initialization infrastructure" could be used
  to replace initialization boilerplate code with automatic children creation and interface delegating.
  ** At component level dependencies and children could be configured with lifetime CLR-attributes
     like [Singleton/Global], [Instance], or [Call (with complex proxy)]
* (?) "Component initialization infrastructure could use optional "test context"
  to replace children components with test stubs/mocks for unit tests.

