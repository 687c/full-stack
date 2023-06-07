# Challenge 2: Implementing API endpoint

## Description
_Knock, knock_. It's the frontend developers. They are about to start developing a search bar and want to start sending user search queries to the backend. But we don't have an endpoint for it!

In this challenge you will need to add an `actix-web` endpoint for search. It needs to accept a search string passed from the FE and return a list of MPNs including what fragments of the MPN should be highlighted. 

It's completely up to you to design the API interface. In [search.rs](search.rs) you will find we have only added the basic skeleton for the endpoint and a way test your endpoint, so you can jump straight to the interesting part.

We also pass a mock API object, which has a search method which you can use to mock sending a request to external API.

## Summary
* Implement an endpoint for search
* It only needs to accept one search string as argument
* It should return a list of parts with MPNs and respective highlighting information
* See [search.rs](search.rs) for the start of your endpoint method
* It is up to you which HTTP methods to use and how, but we would appreciate a comment
 explaining the reasoning behind your API design. 

