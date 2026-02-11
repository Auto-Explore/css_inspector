# css/css-anchor-position/position-area-scrolling-002.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-scrolling-002.tentative.html"
}
```

## style[0]

```css

  .pos {
    position: absolute;
    box-sizing: border-box;
    border: solid;
    position-anchor: --anchor;
    width: 50%;
    height: 50%;
    background: cyan;
  }
  #container.thicker > .pos {
    border-width: thick;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
