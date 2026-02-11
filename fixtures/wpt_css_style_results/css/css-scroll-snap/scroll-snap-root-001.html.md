# css/css-scroll-snap/scroll-snap-root-001.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-snap-root-001.html"
}
```

## style[0]

```css

html, body { margin: 0; padding: 0; }

:root {
  scroll-snap-type: block mandatory;
  scroll-padding: 25%;
  overflow: hidden; /* hide scrollbars for reftest analysis */
}

#fail {
  font: bold 2em;
  background: red;
  height: 120vh;
  margin-bottom: 60vh;
}

#target {
  margin-bottom: 120vh;
  scroll-margin: 25vh;
  scroll-snap-align: start;
  border-top: solid blue;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
