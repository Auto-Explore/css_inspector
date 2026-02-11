# css/css-anchor-position/position-area-basic.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-basic.html"
}
```

## style[0]

```css

  #container {
    position: absolute;
    width: 400px;
    height: 400px;
    margin: 0 auto;
    border: 2px solid;
    background: #eee;
  }
  #anchored {
    position: absolute;
    align-self: stretch;
    justify-self: stretch;
    position-anchor: --anchor;
    background: #FA08;
    outline: 1px solid orange;
  }
  #anchor {
    margin-top: 150px;
    margin-left: 100px;
    width: 150px;
    height: 75px;
    anchor-name: --anchor;
    background: blue;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “outline”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
