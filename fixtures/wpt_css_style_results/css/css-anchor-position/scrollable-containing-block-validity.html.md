# css/css-anchor-position/scrollable-containing-block-validity.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/scrollable-containing-block-validity.html"
}
```

## style[0]

```css

.scroller {
  /* Use overflow hidden instead of scroll to mitigate scrollbar differences. */
  overflow: hidden;
  position: relative;
  width: 80px;
  height: 80px;
  margin: 10px;
  border: solid 3px;
  padding: 10px;
}

.filler {
  min-width: 180px;
  min-height: 180px;
}

.anchor {
  anchor-name: --a;
}

.target {
  position: absolute;
  inset: 0;
  place-self: stretch;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
