# css/css-scroll-snap/scroll-snap-root-002.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-snap-root-002.html"
}
```

## style[0]

```css

html, body { margin: 0; padding: 0; }

:root {
  scroll-snap-type: block mandatory;
  overflow: hidden; /* hide scrollbars for reftest analysis */
}

body {
  scroll-padding: 25%;
}

#fail {
  height: 120vh;
  font: bold 2em;
  background: red;
}

#target {
  margin: 120vh 0;
  scroll-snap-align: end;
  border-bottom: solid orange thick;
  height: 20px; /* Avoid subpixel sizes depending on fonts */
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “font”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
