# css/css-shapes/shape-outside/shape-image/gradients/shape-outside-linear-gradient-016.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-image/gradients/shape-outside-linear-gradient-016.html"
}
```

## style[0]

```css

    .container {
      writing-mode: vertical-lr;
      text-orientation: sideways;
      inline-size: 100px;
      block-size: 200px;
      background-color: red;
      font-family: Ahem;
      font-size: 50px;
      line-height: 1;
    }
    #test {
      color: green;
    }
    #float-left {
      /* Note: In .container's writing-mode, "float: left" actually floats
         us towards the top. */
      float: left;
      inline-size: 100px;
      block-size: 200px;
      background: linear-gradient(to bottom, green 50%, transparent 50%);
      shape-outside: linear-gradient(to bottom, green 25%, transparent 25%);
      shape-margin: 25%;
    }
    #float-right {
      /* Note: In .container's writing-mode, "float: right" actually floats
         us towards the bottom. */
      float: right;
      inline-size: 100px;
      block-size: 200px;
      background: linear-gradient(to top, green 50%, transparent 50%);
      shape-outside: linear-gradient(to top, green 5%, transparent 5%);
      shape-margin: 45%
    }
    
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “shape-margin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “shape-margin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
