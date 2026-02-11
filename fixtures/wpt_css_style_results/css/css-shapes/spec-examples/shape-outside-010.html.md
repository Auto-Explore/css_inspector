# css/css-shapes/spec-examples/shape-outside-010.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/spec-examples/shape-outside-010.html"
}
```

## style[0]

```css

        .container {
          font-family: Ahem;
          font-size: 20px;
          line-height: 2em;
        }
        #test {
            width: 400px;
            color: green;
        }
        #image {
            float: left;
            shape-outside: url("support/circle-no-shadow.png");
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
