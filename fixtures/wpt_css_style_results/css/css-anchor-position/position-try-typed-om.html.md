# css/css-anchor-position/position-try-typed-om.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-typed-om.html"
}
```

## style[0]

```css

  @position-try --pt {
    left: anchor(right);
    top: 50px;
    width: anchor-size(width);
    height: 40px;
  }
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
    left: 9999px; /* force overflow */
    position-try-fallbacks: --pt;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
