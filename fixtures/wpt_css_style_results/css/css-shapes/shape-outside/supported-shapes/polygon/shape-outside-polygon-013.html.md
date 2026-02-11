# css/css-shapes/shape-outside/supported-shapes/polygon/shape-outside-polygon-013.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/polygon/shape-outside-polygon-013.html"
}
```

## style[0]

```css

        body {
            margin: 0;
        }
        #container {
            position: absolute;
            top: 50px;
            left: 10px;
            font-size: 20px;
            font-family: Ahem;
            line-height: 20px;
            width: 240px;
            height: 240px;
            background-color: red;
            color: green;
        }
        #test-shape {
            float: right;
            width: 160px;
            height: 160px;
            margin: 10px;
            border: 10px solid transparent;
            padding: 10px;
            shape-margin: 10px;
            shape-outside: border-box polygon(20% 20%, 100% 20%, 100% 90%, 50% 90%, 50% 70%, 70% 70%, 70% 40%, 20% 40%);
        }
        .ref-shape {
            position: absolute;
            background-color: green;
        }
        #ref-1 {
            top: 90px;
            left: 70px;
            width: 180px;
            height: 60px;
        }
        #ref-2 {
            top: 150px;
            left: 170px;
            width: 80px;
            height: 40px;
        }
        #ref-3 {
            top: 190px;
            left: 130px;
            width: 120px;
            height: 60px;
        }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “shape-margin”.",
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
