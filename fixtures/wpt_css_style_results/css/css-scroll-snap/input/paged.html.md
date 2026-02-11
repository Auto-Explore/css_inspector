# css/css-scroll-snap/input/paged.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/input/paged.html"
}
```

## style[0]

```css

html, body {
  margin: 0;
}
.scroller {
  height: 100vh;
  overflow: auto;
  position: relative;
  scroll-snap-type: y mandatory;
  counter-reset: --page;
}

.gap {
  height: 100vh;
}

.page {
  counter-increment: --page;
  height: 90vh;
  scroll-snap-align: center;
  padding: 8px;
  position: relative;
  --page: counter(--page);
}
.short {
  height: 25vh;
}
.page > div::before {
  content: "Page " counter(--page);
  font-size: 1.5em;
}
.page > div {
  box-sizing: border-box;
  border: 3px solid black;
  border-radius: 5px;
  overflow: clip; /* Make sure font size doesn't cause pages to be larger than expected. */
  padding: 8px;
  height: 100%;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
