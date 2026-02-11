# css/css-anchor-position/position-try-transition-basic.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-transition-basic.html"
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
    width: 300px;
  }
  #target {
    position: absolute;
    left: 300px;
    width: 50px;
    height: 50px;
    background: skyblue;
  }
  #target.fallback {
    position-try-fallbacks: --200;
  }
  #target.anim {
    transition: left 1000s steps(2, start);
  }
  @position-try --200 {
    left: 200px;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown at-rule.",
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
