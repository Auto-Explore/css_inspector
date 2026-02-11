# css/cssom-view/position-sticky-root-scroller-with-scroll-behavior.html

```json
{
  "format_version": 3,
  "file": "css/cssom-view/position-sticky-root-scroller-with-scroll-behavior.html"
}
```

## style[0]

```css

html {
  scroll-behavior: smooth;
}

body {
  /* Assumption: 3000px is taller than any user agents test window size. */
  height: 3000px;
  /* A property which propagates for <html>. */
  overflow-x: hidden;
}

#sticky {
  position: sticky;
  top: 50px;
  width: 200px;
  height: 200px;
  background-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
