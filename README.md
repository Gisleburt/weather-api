Weather API
===========

A GraphQL wrapper around the MetOffice API.

WIP Notes
---------

The MetOffice API is feature rich but very difficult for normal people like me to use. Its also missing a few details
such as pollen count, which I wanted to add (this will be done by scraping the website html). A GraphQL interface
seemed like a good way to do this.

You will need to provide this API with your met office api token as it queries that API underneath. Never give your API
key to someone you don't trust. This API is provided to be run by you, do not let someone MITM your API key.  
