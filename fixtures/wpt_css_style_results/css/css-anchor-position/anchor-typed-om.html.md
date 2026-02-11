# css/css-anchor-position/anchor-typed-om.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-typed-om.html"
}
```

## style[0]

```css

  #cb {
    position: relative;
    width: 100px;
    height: 100px;
    border: 1px solid black;
  }
  #anchor {
    position: absolute;
    left: 10px;
    top: 10px;
    width: 20px;
    height: 20px;
    background: coral;
    anchor-name: --a;
  }
  #target {
    position: absolute;
    background: skyblue;
    position-anchor: --a;
    left: anchor(right);
    top: calc(anchor(bottom) + 5px);
    width: anchor-size(--unknown width, 25px);
    height: calc(anchor-size(height) * 2);
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
