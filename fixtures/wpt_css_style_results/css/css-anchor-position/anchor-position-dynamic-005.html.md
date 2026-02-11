# css/css-anchor-position/anchor-position-dynamic-005.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-dynamic-005.html"
}
```

## style[0]

```css

  #cb {
    position: relative;
    width: 100px;
    height: 100px;
    overflow: clip;
  }
  #anchor {
    position: absolute;
    inset: 0;
    background: green;
    width: 100px;
    height: 100px;
    anchor-name: --a;
  }
  #bug {
    position: absolute;
    width: 100px;
    height: 100px;
    background: red;
    left: anchor(right);
    top: anchor(top);
    position-anchor: --a;
  }
  #target {
    position: absolute;
    top: anchor(top);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
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
