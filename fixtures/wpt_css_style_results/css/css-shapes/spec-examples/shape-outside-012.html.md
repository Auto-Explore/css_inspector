# css/css-shapes/spec-examples/shape-outside-012.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/spec-examples/shape-outside-012.html"
}
```

## style[0]

```css

        .container {
          width: 400px;
          font-family: Ahem;
          font-size: 20px;
          line-height: 2em;
        }
        #test {
            color: green;
        }
        #test-image {
            float: left;
            shape-outside: url("support/circle-shadow.png");
            shape-image-threshold: 0.9;
        }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “shape-image-threshold”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
