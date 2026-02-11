# css/css-shapes/shape-outside/assorted/reference/float-should-push-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/assorted/reference/float-should-push-ref.html"
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
    clear: left;
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
  }

  .polygon {
    background: pink;
  }

  .image {
    background: brown;
  }

```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
