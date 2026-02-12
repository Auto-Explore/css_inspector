# css/css-anchor-position/try-tactic-back-to-base.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/try-tactic-back-to-base.html"
}
```

## style[0]

```css

  #cb {
    position: absolute;
    width: 400px;
    height: 200px;
    border: 1px solid black;
  }
  #target {
    position: absolute;
    width: 50px;
    height: 50px;
    /* Does not fit ... */
    left: 180px;
    top: 190px;
    /* ... and neither does this. */
    position-try-fallbacks: flip-block;
    background-color: coral;

  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
