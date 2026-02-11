# css/css-anchor-position/position-area-scroll-adjust.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-scroll-adjust.html"
}
```

## style[0]

```css

  #scroller {
    width: 400px;
    height: 200px;
    overflow: auto;
  }
  #anchor {
    anchor-name: --a;
    background: green;
    width: 200px;
    height: 100px;
  }
  #anchored {
    position-anchor: --a;
    position-area: bottom;
    margin: 0;
    padding: 0;
    border: none;
    background: green;
    width: 200px;
    height: 100px;
  }
  .filler { height: 200px; }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
