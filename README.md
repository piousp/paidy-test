## Pablo Peraza Programming Skill Assesment

# How to run this
This is a `Rust` project build with `cargo`. To run,
simply execute

```bash
cargo run
```

to see the output of the application


# First revision questions

1) How can we change this to create a server process?  Currently, the main app creates a data structure for the "restaurant" and then launches a number of clients to bang on the data structure at various times.  It's okay to not use a web server for this test and instead use a direct API, but what will we need to change to get a standalone server thread running?  What implications would arise from this? `DONE`

   > There's a lot of things that need to change in order to create a server:

        * First, we need to have an actual API able to read from external data sources (usually REST nowadays)
        * You probably want to introduce a database; but if that falls out of the scope of the test, at the very least have the Tables and Items have an unique and randomly generated identifier in order to properly support CRUD operations
        * Probably abstract away the data layer from the API layer (which currently is one and the same)
        * Another option, if we want to keep things simple, is to have a `loop` in the main process and have listeners or streams that listen to incomming requests. Those requests can be from network, or files dropped somewhere, or constatly looking into the database for example.

2) Why do all of the functions for the Restaurant return a reference to a vector?  For the get it makes sense, but why the delete and add? `DONE`

   > It was a design decision in order to be able to get the current data from a table after each operation, without having to ask for the status. Obviously you don't actually need that (nor would you want to usually) but it was just a way of reducing the amount of calls I had to do (and to try to keep everything returning the same type; again, just a design decision at the time of writing but it is not necessary).

3) How would we update items in the Restaurant? `DONE`

   > Version 1 didn't really support that operation in itself. What can be done is to remove the item, modify it, and then re-insert it, that's the only way with that code base.

4) How can we change this to safely and consistently allow for deletion of a single order at a table when there are more than one of the same name (which happen to have the same cook time)? `DONE`

   > From answer 1, we need to implement an unique randomly-generated identifier for each dish. Another way of doing it is by using the position of each item as their own identifier.

5) You have an "if/else" in the add_items func and both seem to call very similar code.  Can we make this a bit more functional? `DONE`

   > By introducing a function. Yes, somehow I missed that code repetition on the first try. Was probably too busy trying to get it to work.

6) Why do you insist on cook time being equal to define item equality? `DONE`

   > Since at the time the Items didn't have their own unique identifier, I figured it would be a good idea to consider both the name and the cook time for the equality function. There's no other reason than that.

7) Can we make a service for this, which we can use to expose to a webservice endpoint?  How would that look?  How can we handle having a global data storage in such an environment (and still remain functional)?  What might that mean for returning references in your function calls?  How can we help resolve any problems? `DONE`

   > Yes. Will need to use a library for abstracting the HTTP stuff. Since we are now going to serialize the data into the http response, those references can now be copies or dereferences of the original one (when returning)

8) The main is allowed to just be a simple test, for sure, but as an exercise, is there  a way to make it more flexible?  Like read in some config file, or some parameters?  It seems like that would be a necessary step anyways, if we needed to turn this into a full-fledged server. `DONE`

   > Yes. Usually we want to read `environment variables` for different parameters of the application.
