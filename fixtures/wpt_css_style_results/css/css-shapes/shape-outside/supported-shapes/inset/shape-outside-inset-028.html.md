# css/css-shapes/shape-outside/supported-shapes/inset/shape-outside-inset-028.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/inset/shape-outside-inset-028.html"
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
            text-align: right;
        }
        #test-shape {
            float: right;
            width: 200px;
            height: 200px;
            shape-margin: 10px;
            shape-outside: inset(60px 10px 60px 110px round 20px);
        }
        #static-shape {
            position: absolute;
            left: 100px;
            width: 100px;
            height: 100px;
            top: 50px;
            background-color: green;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
