# css/css-contain/contain-style-counters-003.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-style-counters-003.html"
}
```

## style[0]

```css

  body
    {
      counter-reset: counter-of-span 17;
    }

    /*
    This creates a new counter identified as "counter-of-span"
    and initially sets such counter to 17 (an entirely
    arbitrary number)
    */

  body
    {
      contain: style;
    }

    /*
    This will reset the counter to 0.
    */

  body span
    {
      counter-increment: counter-of-span 4;
    }

    /*
    This increments the counter identified as "counter-of-span"
    of the step value of 4 (an entirely arbitrary number) each
    and every time there is a <span> descendant within the subtree
    of body
    */

  div
    {
      font-size: 3em;
    }

  div::after
    {
      content: counter(counter-of-span);
    }

    /*
    Now, the generated content after the span is set to the
    current value of the counter identified as "counter-of-span"
    */
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
