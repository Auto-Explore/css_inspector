# css/css-anchor-position/position-area-scrolling-003.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-scrolling-003.tentative.html"
}
```

## style[0]

```css

  body {
    overflow: hidden;
    margin: 0;
  }
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
