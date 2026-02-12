# css/css-shapes/shape-outside/supported-shapes/polygon/shape-outside-polygon-007.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/polygon/shape-outside-polygon-007.html"
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
            width: 200px;
            height: 200px;
            background-color: red;
            color: green;
        }
        #test-shape {
            float: left;
            width: 200px;
            height: 200px;
            shape-outside: polygon(0px 40px, 120px 40px, 120px 80px, 80px 80px, 80px 120px, 160px 120px, 160px 160px, 0px 160px);
        }
        .ref-shape {
            position: absolute;
            left: 10px;
            background-color: green;
            height: 40px;
        }
        #ref-1 {
            top: 90px;
            width: 120px;
        }
        #ref-2 {
            top: 130px;
            width: 80px;
        }
        #ref-3 {
            top: 170px;
            width: 160px;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
