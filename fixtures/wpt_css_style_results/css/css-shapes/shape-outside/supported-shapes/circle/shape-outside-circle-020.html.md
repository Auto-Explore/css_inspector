# css/css-shapes/shape-outside/supported-shapes/circle/shape-outside-circle-020.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/supported-shapes/circle/shape-outside-circle-020.html"
}
```

## style[0]

```css

    body {
        margin: 0;
    }
    #container {
        position: relative;
    }
    #test-container {
        font: 40px/1 Ahem, sans-serif;
        width: 300px;
        height: 200px;
        color: green;
    }
    #test-shape {
        float: left;
        width: 100px;
        height: 100px;
        margin: 10px;
        padding: 10px;
        border: 20px solid transparent;
        shape-margin: 10px;
        shape-outside: border-box circle(86px at 40%);
    }
    #line {
        position: absolute;
        top: 0px;
        left: 168px;
        width: 2px;
        height: 200px;
        border-left: 2px solid blue;
    }
    #failure {
        position: absolute;
        top: 80px;
        left: 170px;
        width: 40px;
        height: 40px;
        background-color: red;
        z-index: -1;
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
