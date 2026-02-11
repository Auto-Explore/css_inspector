# css/css-anchor-position/anchor-function-pseudo-element-basic.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-function-pseudo-element-basic.html"
}
```

## style[0]

```css

body { margin: 0 }
#anchor, #target::before, #target::after {
    width: 100px;
    height: 100px;
    position: absolute;
}
#anchor.moved {
    left: 200px;
    top: 200px;
}
#anchor {
    left: 50px;
    top: 100px;
    anchor-name: --a;
    background: blue;
}
#target::before {
    position-anchor: --a;
    left: anchor(right);
    top: anchor(top);
    background: green;
    content:'';
}
#target::after {
    position-anchor: --a;
    left: anchor(left);
    top: anchor(bottom);
    background: green;
    content:'';
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
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
