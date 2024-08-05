# Data Engineering Role Assignment Submission

## Explanation of the task

Logo extraction is mostly HTML parsing. There are a couple of ways web developers display logos:

- Basic img tags
- SVG tags
- CSS Background images
- Various JavaScript methods

The most common ones are basic img tags and SVG tags. The next step is to distinguish the logos from other images. Image tags usually contain hints like "logo", "brand" in them. A well-prepared and optimized list of these hints would be a necessity for production.

The simple algorithm would be to cover the first 2 of the ways above. JavaScript methods would consume a lot of time and would pretty much be inaccurate without good statistics about which methods most developers tend to use in that scenario.

## Feature Ideas

This title contains some hardcore optimization ideas and feature ideas for an imaginary case where this would go to production.

### Custom HTML parser

We could implement a custom HTML parser specifically for this task. It would be slightly faster than using a parser.

### Asynchronous web requests

With non-blocking web requests, we would be able to send multiple requests at once, instead of waiting for each request to complete before continuing.

### Incremental Parsing

For large pages (which is quite common in this age of internet), we could implement "incremental parsing". We would process data as they are received, reducing latency. Of course, this would require tweaking with the HTTP request library a lot. We might even end up implementing a custom HTTP request library.

---

Those are some ideas that would get considered when scaling a system like this to millions.


## Selecting the right logo from filtered images

This title will explain the task of ``url_filter.rs`` (not implemented).

``fetch_html`` returns all candidates that could be the logo in the webpage. ``url_filter.rs`` is responsible with picking the right one.

A simple way to choose the correct image would be a ranking system.

### Ranking System

This ranking system would assign each logo candidate a point to select the right logo. For example, let's think of an example. Let's say we have 3 candidates:

```
<img src="/static/logo1.png" alt="logo othercompany" />
<img src="/static/logo-main.png" alt="logo domain.com main" />
<img src="/static/logo-background.jpeg" alt="logo domain.com background" />
```

In this case, we would have a complicated point system.

| **Point Difference** | **Reason**                              | **Explanation**                                                                                                                                                    |
|----------------------|-----------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| -0.5                 | Contains hint "background"              | If the candidate contains "background", it is less likely that it is the right choice.                                                                             |
| 1.5                  | Contains the domain name of the website | This makes the candidate more likely to be the right logo                                                                                                          |
| 0.1                  | Contains hint "main"                    | This slightly increases the chances that this is the right choice, since the webpage can contain multiple logos and the one with "main" is probably the right one. |

Rating each candidate with this system, we would get:

| **Candidate**    | **Point** |
|------------------|-----------|
| First candidate  | 0         |
| Second candidate | 1.6       |
| Third candidate  | 1.0       |

Of course, this is just an example. Fine-tuning the point system would take time, and would require a lot of data.

### Ranking logos by their resolutions

We could also implement an optimized system that would get the resolution of given URL. Then we would use this data to rank each logo.

- An image that is closer to a shape of a square is more likely to be a good logo.
- An image that has high resolution and has the shape of a horizontal rectangle (e.g. 1000\*800) is less likely to be a good logo.
- An image that has very low resolution is less likely to be a good logo.

This approach focuses on increasing the accuracy of the system.
