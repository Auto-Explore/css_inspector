# css/css-scroll-snap/scroll-snap-root-003.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-snap-root-003.html"
}
```

## style[0]

```css

:root {
  overflow: hidden; /* hide scrollbars for reftest analysis */
}

body {
  scroll-snap-type: block mandatory;
}

#pass {
  height: 120vh;
}

#target {
  scroll-snap-align: start;
  height: 100vh;
  background: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
