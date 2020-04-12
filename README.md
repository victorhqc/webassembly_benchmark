# webassembly_benchmark

Checking performance using a big table, with some operations like sorting using WebAssembly vs JS

## Motivation

I've been a Web Developer for years, and my most used language so far is JS. It has its flaws as
any other language does, but this language is quite useful in many scenarios. Using it in the
backend with Node.js for example, is quite fast if it's used correctly, it basically performs as
C++ code.

However, JS has its limitations. Is not very good at intense CPU, and pretty much anything other
than I/O or Event Driven Programming. Now that most applications are connected to the Internet,
it makes sense to explore different languages to satisfy our clients.

Take the native Apps from our phones for example. There are amazing apps that everybody loves,
like Google Maps. Have you tried to use Google maps Web App in your browser? Nobody does that,
rather use the Native Application because the UX is so much better.

But then there are the useless throw-away apps like for conference applications. You download
20/40 MB for an application that you're gonna use 2 days and might stay in your Phone for much
longer.

Why not use a Web App for that? Well, most WebApps in Phones are just bad, slow and not so nice to
use. Here's where WebAssembly might come in handy.

This project will try to assess the performance gains by writing a WebAPp using WebAssembly.
