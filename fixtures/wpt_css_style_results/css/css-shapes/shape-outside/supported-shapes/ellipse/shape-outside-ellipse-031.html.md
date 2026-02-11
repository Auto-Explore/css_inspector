# css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-031.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/ellipse/shape-outside-ellipse-031.html"
}
```

## style[0]

```css

    body {
        margin: 0;
    }
    .container {
        font: 15px Ahem, sans-serif;
        line-height: 20px;
        width: 400px;
        height: 200px;
        color: green;
    }
    .ellipse {
        width: 160px;
        height: 160px;
        shape-outside: ellipse(50% 25% at 50% 25%);
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
