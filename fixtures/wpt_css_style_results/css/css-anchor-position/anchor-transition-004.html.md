# css/css-anchor-position/anchor-transition-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-transition-004.html"
}
```

## style[0]

```css

  body { margin: 0; }
  #anchor {
    anchor-name: --a;
    width: 100px;
    height: 100px;
    background: teal;
  }
  #anchored {
    position: absolute;
    position-anchor: --a;
    left: anchor(right, 17px);
    top: anchor(top, 17px);
    width: 100px;
    height: 100px;
    background: orange;
    transition: left 2s;
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
