# css/css-transforms/3d-point-mapping-origins.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/3d-point-mapping-origins.html"
}
```

## style[0]

```css

  body {
    margin: 0;
    border: 1px solid black;
  }

  .test {
    display: inline-block;
    height: 200px;
    width: 200px;
    border: 1px solid black;
    margin: 20px;
  }

  .perspective-container {
    height: 140px;
    width: 140px;
    margin: 20px;
    border: 1px solid black;
    box-sizing: border-box;
    perspective: 400px;
    perspective-origin: 20% 80%;
  }

  .transformed-parent {
    position: relative;
    height: 100px;
    width: 100px;
    padding: 20px;
    margin: 20px;
    border: 1px solid black;
    box-sizing: border-box;
    transform: translateZ(100px) rotateY(-40deg);
    transform-origin: 20% 40%;
  }

  .light-gray-box {
    background-color: #DDD;
  }

  .medium-gray-box {
    background-color: #AAA;
  }

  .blue-box {
    height: 100px;
    width: 100px;
    box-sizing: border-box;
    background-color: blue;
    border: 1px solid black;
  }

  .green-box {
    height: 100px;
    width: 100px;
    padding: 20px;
    box-sizing: border-box;
    background-color: #C0D69E;
    border: 1px solid black;
  }

  .layered {
    position: relative;
  }

  div[id]:hover {
    outline: 3px solid orange;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “perspective-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “outline”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
