# css/css-contain/contain-strict-011.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-strict-011.html"
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

  div
    {
      contain: strict;
    }

  div > span
    {
      background-color: yellow;
      color: red;
      counter-increment: counter-of-span 3;
    }

    /*
    This increments the counter identified as "counter-of-span"
    of the step value of 3 (an entirely arbitrary number) each
    and every time there is a <span> child within the subtree
    of div
    */

  p#test::after
    {
      content: counter(counter-of-span);
      font-size: 3em;
    }

    /*
    Now, the generated content is set to the current
    value of the counter identified as "counter-of-span":
    17 + 3 * 3 == 26
    */
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
