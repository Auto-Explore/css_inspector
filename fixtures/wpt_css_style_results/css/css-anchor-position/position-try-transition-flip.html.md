# css/css-anchor-position/position-try-transition-flip.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-transition-flip.html"
}
```

## style[0]

```css

  #cb {
    display: inline-block;
    position: relative;
    width: 400px;
    height: 250px;
    border: 1px solid black;
  }
  #cb.shrink {
    width: 325px;
  }
  #anchor {
    position: absolute;
    width: 50px;
    height: 50px;
    background: coral;
    left: 250px;
    top: 50px;
    anchor-name: --a;
  }
  #target {
    position-anchor: --a;
    position: absolute;
    left: anchor(right);
    top: anchor(top);
    width: 50px;
    height: 50px;
    background: skyblue;
  }
  #target.fallback {
    position-try-fallbacks: flip-start;
  }
  #target.anim {
    transition: left 1000s steps(2, start);
  }
```

```json
{
  "errors": 5,
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
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
