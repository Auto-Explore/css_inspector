# css/css-shapes/shape-outside/assorted/float-retry-push-inset.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/assorted/float-retry-push-inset.html"
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
    shape-outside: inset(0px 40px 40px 0px round 0 0 34.142136px 0);
    /* 34.142136 = 10 / (1 - (sqrt(2) / 2)) */
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
