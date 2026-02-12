# css/css-shapes/shape-outside/assorted/float-retry-push-circle.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/assorted/float-retry-push-circle.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
    line-height: 1;
  }

  #too-wide {
    display: inline-block;
    height: 20px;
    width: 250px;
    background: blue;
  }

  #shape {
    width: 100px;
    height: 100px;
    float: left;
    shape-outside: circle(70.710678px at 0px 0px);
    /* 70.710678 = 50 / (sqrt(2) / 2) */
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
