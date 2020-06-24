# Nice findings
These nice findings were found after implementing with C#:
1. At first sight, we can use DI to join all the components. But as soon as we need a small piece of 
   unregular scheme (e.g., we need to provide two different IPLayer links to GameFlow), 
   the classic DI became broken and we have to construct and link components by hands.
2. Circular component links (without circular type dependencies!) is hard to construct 
   without special intermediate adapters. Whereas with declarative description 
   (just as it is drawn on the diagram) it could be possibly done automatically.
   And I should say, that it isn't clear, that we should avoid such *runtime* circular component 
   relations. 
3. Interface delegating should be done by hands, and it's awful.
4. C# and DI don't allow to create objects with partially specified dependencies from parent components. 
   Consequenlty, as soon as we decided to construct non-trivial component dependencies by hands, 
   we must provide *all* the dependencies. Although, maybe, this should be called "explicit component schema".
   May be, the more appropriate approach would be if we used ServiceProvider as an explicit dependencies source,
   for all constructors. 
5. Actually, all these difficulties reveal the fact that usual IoC/DI practice confronts with component 
   approach, where *nobody* (including magical `Startup` mega-component!) can reach the internals of some
   component and control the interconnectrions of that internals. Instead, each of the component 
   should manage it's internals by itself, and traditional ServiceCollection/ServiceProvider can't help here.
   But maybe we can develop another helper tools, which will reduce the amount of boilerplate code.
6. It seems that in component approach, as opposite to DI, component *creates* all it's inernals by itself
   in constructor. But, of course, we can use some means to configure *variants* of types and interconnections
   by some kind of "component's configuration" (which shopuld probably be opaque for all other components and code parts).
   Helper tools can use component hierarchy information, fields information and that configuration, to help create
   and bind internals automatically, with particular thin customization, if they are.
7. At the same time, all external dependencies shoud, as it was in DI/IoC practice, still be injected as constructor parameters.


