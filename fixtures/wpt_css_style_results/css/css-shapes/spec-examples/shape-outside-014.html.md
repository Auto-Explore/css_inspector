# css/css-shapes/spec-examples/shape-outside-014.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/spec-examples/shape-outside-014.html"
}
```

## style[0]

```css

        #test {
            color: green;
            position: relative;
            width: 400px;
            font-family: Ahem;
            font-size: 20px;
            line-height: 2em;
        }
        #box {
            float: left;
            width: 120px;
            height: 120px;
            background-color: lightblue;
            margin: 40px;
            border-radius: 90px;
            shape-outside: margin-box;
        }
        #border-shape {
            position: absolute;
            top: 0px;
            left: 0px;
            width: 160px;
            height: 160px;
            background-color: lightblue;
            border: 20px solid lightblue;
            border-radius: 90px;
            z-index: -1;
        }
    
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “shape-outside”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
