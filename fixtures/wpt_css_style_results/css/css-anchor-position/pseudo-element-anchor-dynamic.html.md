# css/css-anchor-position/pseudo-element-anchor-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/pseudo-element-anchor-dynamic.html"
}
```

## style[0]

```css

  .cb {
    position: relative;
    width: 200px;
    height: 200px;
  }
  #anchor1.enabled::before {
    display: block;
    width: 100px;
    height: 100px;
    content: "";
    anchor-name: --a1;
    background: blue;
  }
  #anchor2.enabled::after {
    display: block;
    width: 100px;
    height: 100px;
    content: "";
    anchor-name: --a2;
    background: pink;
  }
  .anchored {
    position: absolute;
    width: 100px;
    height: 100px;
    left: anchor(right);
    top: anchor(bottom);
    background: orange;
  }
  #anchored1 { position-anchor: --a1; }
  #anchored2 { position-anchor: --a2; }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
