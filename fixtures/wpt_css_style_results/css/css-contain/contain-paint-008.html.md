# css/css-contain/contain-paint-008.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-paint-008.html"
}
```

## style[0]

```css

rt {
  contain: paint;
  display: ruby-text;
  font-size: 1rem;
  font-family: monospace;
}
rt::after {
  content: "PASS";

  /* Doing the following instead of position:absolute to move it out into the area that would be
     clipped because Firefox clips absolutely positioned content of rt even in builds without
     support for css containment.
     Since this technique works also, there's no need to trigger a false negative.
   */
  position: relative;
  left: 4ch;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
