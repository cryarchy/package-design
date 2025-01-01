# Recognizing violations of the single responsibility principle

Symptoms of a class that may violate the SRP:

-   The class has many instance variables
-   The class has many public methods
-   Each method of the class uses different instance variables
-   Specific tasks are delegated to private methods

The above are good reasons to extract "collaborator classes" from the class to which some of the class' responsibilities are delegated thereby making the class adhere to the SRP.
