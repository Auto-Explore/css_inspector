# css/css-shapes/spec-examples/shape-outside-017.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/spec-examples/shape-outside-017.html"
}
```

## style[0]

```css

        #test {
            color: green;
            width: 400px;
            font-family: Ahem;
            font-size: 20px;
            line-height: 2em;
        }
        #box {
            float: left;
            width: 200px;
            height: 200px;
            background-color: lightblue;
            border-radius: 90px;
            shape-outside: content-box;
        }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
