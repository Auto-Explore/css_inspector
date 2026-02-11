# css/css-shapes/spec-examples/shape-outside-013.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/spec-examples/shape-outside-013.html"
}
```

## style[0]

```css

        #test {
            position: relative;
            width: 400px;
            color: green;
            font-family: Ahem;
            font-size: 20px;
            line-height: 2em;
        }
        #test-image {
            float: left;
            shape-outside: url("support/circle-no-shadow.png");
            shape-margin: 15px;
        }
        #margin-circle {
            position: absolute;
            top: -15px;
            left: -15px;
            width: 230px;
            height: 230px;
            background-color: blue;
            border-radius: 150px;
            z-index: -1;
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
      "message": "Unknown property “shape-margin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
