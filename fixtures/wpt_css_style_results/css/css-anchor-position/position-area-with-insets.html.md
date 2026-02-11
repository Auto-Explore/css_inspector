# css/css-anchor-position/position-area-with-insets.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-with-insets.html"
}
```

## style[0]

```css

  #container {
    position: absolute;
    width: 400px;
    height: 400px;
  }
  #anchored {
    position: absolute;
    align-self: stretch;
    justify-self: stretch;
    position-anchor: --anchor;
  }
  #anchor {
    margin-top: 150px;
    margin-left: 100px;
    width: 150px;
    height: 75px;
    anchor-name: --anchor;
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
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
