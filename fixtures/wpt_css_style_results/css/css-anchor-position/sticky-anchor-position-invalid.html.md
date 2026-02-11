# css/css-anchor-position/sticky-anchor-position-invalid.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/sticky-anchor-position-invalid.html"
}
```

## style[0]

```css

#scroll-container {
  width: 200px;
  height: 200px;
  overflow-y: scroll;
}

#scroller {
  height: 400px;
}

#sticky {
  position: sticky;
  height: 150px;
  background: green;
  top: anchor(--invalid top, 42px); /* Should use the fallback */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
