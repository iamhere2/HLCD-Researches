# Nice findings
These nice findings were found after/during implementing ChessApp with C#:
1. We should avoid using traditional DI/IoC approach with a single `Startup` point, responsible for all type registration
1.1. At first sight, we can use DI to join all the components. But as soon as we need a small piece of 
     unregular scheme (e.g., we need to provide two different IPLayer links to GameFlow), 
     the classic DI became broken and we have to construct and link components by hands.
1.2. Circular component links (without circular type dependencies!) are hard to construct 
     without special intermediate adapters. Whereas with a declarative description 
     (just as it is drawn on the diagram) it could be possibly done automatically.
     And I should say, that it isn't clear, that we should avoid such *runtime* circular component 
     relations. 
1.3. C# and DI don't allow to create objects with partially specified dependencies from parent components. 
     Consequently, as soon as we decided to construct non-trivial component dependencies by hands, 
     we must provide *all* the dependencies. Although, maybe, this should be called "explicit component schema".
     May be the more appropriate approach would be if we used ServiceProvider as an explicit dependencies source,
     for all constructors? 
1.4. But actually, all these difficulties reveal the fact that usual IoC/DI practice confronts with the component 
     approach, where *nobody* (including magical `Startup` mega-component!) can reach the internals of some
     component and control the interconnections of that internals. Instead, each of the components
     should manage its internals by itself, and traditional ServiceCollection/ServiceProvider can't help here.
     But maybe we can develop other helper tools, which will reduce the amount of boilerplate code.
1.5. It seems that in component approach, as opposite to DI, component *creates* all its inernals by itself
     in a constructor. But, of course, we can use some means to configure *variants* of types and interconnections
     by some kind of "component's configuration" (which should probably be opaque for all other components and code parts).
     Helper tools can use component hierarchy information, fields information and that configuration, to help create
     and bind internals automatically, with particular thin customization, if they are.
1.6. At the same time, all external dependencies shoud, as it was in DI/IoC practice, still be injected as constructor parameters.
2. Complex value serialization is quite difficult and hard-predictable despite the fact values should be 
   transparent and safe to (de)serialization. This is usual unfamous flaw in all mainstream OOP-languages:
   values are not ditinguished clearly from other types, and can't give more guerantees due to that.
3. Interface delegating should be done by hands, and it's awful.

