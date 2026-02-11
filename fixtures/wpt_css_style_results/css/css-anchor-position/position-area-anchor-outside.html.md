# css/css-anchor-position/position-area-anchor-outside.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-anchor-outside.html"
}
```

## style[0]

```css

  #container {
    position: relative;
    width: 400px;
    height: 400px;
    margin: 0 auto;
    border: 2px solid;
    background: #eee;
  }
  #anchor {
    position: absolute;
    left: -200px;
    top: 500px;
    width: 100px;
    height: 100px;
    anchor-name: --anchor;
    background: blue;
  }
  #anchored {
    position: absolute;
    align-self: stretch;
    justify-self: stretch;
    position-anchor: --anchor;
    background: #FA08;
    outline: 1px solid orange;
  }
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
      "message": "Invalid value for property “outline”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
