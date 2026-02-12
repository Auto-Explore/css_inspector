# css/css-anchor-position/position-try-pseudo-element.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-pseudo-element.html"
}
```

## style[0]

```css

  #container {
    background: maroon;
    position: relative;
    width: 195px;
    height: 200px;
  }
  #target::before {
    position-try-fallbacks: --f1, --f2;
    background: lime;
    position: absolute;
    left: 200px;
    top: 200px;
    width: 100px;
    height: 200px;
    content: "";
  }
  #target::after {
    position-try-fallbacks: --f1, --f2;
    background: green;
    position: absolute;
    left: 100px;
    width: 100px;
    height: 100px;
    content: "";
  }
  @position-try --f1 {
    top: 100px;
    left: 50px;
  }
  @position-try --f2 {
    top: 0px;
    left: 0px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
