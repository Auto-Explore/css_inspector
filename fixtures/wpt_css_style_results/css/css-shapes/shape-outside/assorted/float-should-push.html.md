# css/css-shapes/shape-outside/assorted/float-should-push.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/assorted/float-should-push.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
    line-height: 1;
  }

  .container {
    width: 300px;
    height: 100px;
    background: lightgray;
  }

  .too-wide {
    display: inline-block;
    width: 250px;
    height: 20px;
    background: lightblue;
  }

  .shape {
    background: black;
    width: 100px;
    height: 100px;
    float: left;
  }

  .spacer {
    clear: left;
    height: 30px;
  }

  .inset {
    background: orange;
    shape-outside: inset(1px 0px 0px 0px);
  }

  .polygon {
    background: pink;
    shape-outside: polygon(0px 0px, 100px 0px, 100px 100px, 0px 100px);
  }

  .image {
    background: brown;
    shape-outside: linear-gradient(0deg, black, black 100%);
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
