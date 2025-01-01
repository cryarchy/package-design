# The Single Responsibility Principle (SRP)

    A class should have one, and only one, reason to change

# Recognizing violations of the single responsibility principle

Symptoms of a class that may violate the SRP:

-   The class has many instance variables
-   The class has many public methods
-   Each method of the class uses different instance variables
-   Specific tasks are delegated to private methods

The above are good reasons to extract "collaborator classes" from the class to which some of the class' responsibilities are delegated thereby making the class adhere to the SRP.

By utilizing the single responsibility principle, you end up with smaller classes that are easier to test, grasp, and maintain and that have fewer dependencies.

SRP corresponds to the **Common Closure** principle that applies to package design. Similarities:

-   Reduction in the number of responsibilities.
-   Results in a smaller number of dependencies and thus reduces coupling.
