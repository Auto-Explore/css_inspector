# css/css-shapes/shape-outside/supported-shapes/inset/shape-outside-inset-010.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/inset/shape-outside-inset-010.html"
}
```

## style[0]

```css

        #container {
            position: relative;
            margin-left: 25px;
        }
        #test-container {
            width: 200px;
            height: 200px;
            font: 25px/1 Ahem;
            background-color: red;
            color: green;
        }
        #test-shape {
            float: left;
            width: 200px;
            height: 200px;
            shape-outside: inset(50px 100px 50px 0px);
        }
        #static-shape {
            position: absolute;
            left: 0px;
            width: 100px;
            height: 100px;
            top: 50px;
            background-color: green;
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
