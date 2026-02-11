# css/css-anchor-position/anchor-getComputedStyle-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-getComputedStyle-004.html"
}
```

## style[0]

```css

  #container {
    position: relative;
    width: 300px;
    height: 300px;
  }
  #anchor {
    anchor-name: --a;
    margin-left: 80px;
    margin-top: 130px;
    width: 100px;
    height: 100px;
  }
  .anchored {
    position: absolute;
    position-anchor: --a;
    position-area: center center;
  }
  #t1 {
    inset: 10px;
  }
  #t2 {
    inset: auto;
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
