# css/css-anchor-position/position-area-ambiguous-self.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-ambiguous-self.html"
}
```

## style[0]

```css

  #abs-container {
    writing-mode: vertical-lr;
    position: absolute;
    width: 400px;
    height: 400px;
  }
  #anchor {
    anchor-name: --a;
    width: 100px;
    height: 100px;
  }
  #anchored {
    position: absolute;
    writing-mode: horizontal-tb;
    direction: rtl;
    position-anchor: --a;
    position-area: self-end self-start;
    width: 100px;
    height: 100px;
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
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
