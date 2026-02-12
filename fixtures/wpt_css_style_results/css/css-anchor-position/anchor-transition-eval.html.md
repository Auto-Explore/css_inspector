# css/css-anchor-position/anchor-transition-eval.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-transition-eval.html"
}
```

## style[0]

```css

  .cb {
    display: inline-block;
    position: relative;
    width: 250px;
    height: 250px;
    border: 1px solid black;
  }
  .anchor {
    position: absolute;
    width: 50px;
    height: 40px;
    left: 60px;
    top: 40px;
    background: coral;
    anchor-name: --a;
  }
  .shift {
    left: 80px;
  }
  .grow {
    width: 70px;
  }
  .anchored {
    position: absolute;
    width: anchor-size(width);
    height: 40px;
    position-anchor: --a;
    top: anchor(bottom);
    left: anchor(right);
    transition-duration: 1000s;
    transition-timing-function: steps(2, start);
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
