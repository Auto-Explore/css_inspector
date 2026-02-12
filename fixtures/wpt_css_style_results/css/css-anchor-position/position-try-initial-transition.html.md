# css/css-anchor-position/position-try-initial-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-initial-transition.html"
}
```

## style[0]

```css

  #cb {
    position: relative;
    width: 100px;
    height: 100px;
    background: lightpink;
  }
  #abs {
    position: absolute;
    background: darkcyan;
    top: 10px;
    left: 10px;
    width: 150px; /* force fallback */
    height: 25px;
    position-try-fallbacks: --pf;
    transition-property: top, left;
    transition-duration: 10s;
    transition-timing-function: steps(2, start);
  }
  @position-try --pf {
    width: 50px;
    top: 50px;
    left: 50px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
