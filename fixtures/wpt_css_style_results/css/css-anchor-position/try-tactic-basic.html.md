# css/css-anchor-position/try-tactic-basic.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/try-tactic-basic.html"
}
```

## style[0]

```css

  @position-try --pf {
    left:10px;
    top:20px;
  }
  #cb {
    position: absolute;
    width: 400px;
    height: 400px;
    border: 1px solid black;
  }
  #target {
    position: absolute;
    left: 99999px; /* force fallback */
    width: 30px;
    height: 40px;
    background-color: skyblue;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
