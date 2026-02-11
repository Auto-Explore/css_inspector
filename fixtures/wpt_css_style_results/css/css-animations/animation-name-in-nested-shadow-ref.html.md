# css/css-animations/animation-name-in-nested-shadow-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-name-in-nested-shadow-ref.html"
}
```

## style[0]

```css


  div {
    position: absolute;
    left: 0;
    top: 0;
  }

  #red {
    width: 300px;
    height: 300px;
    background-color: red;
  }

  #lightgreen {
    width: 300px;
    height: 100px;
    background-color: lightgreen;
  }

  #green {
    left: 100px;
    top: 100px;
    width: 200px;
    height: 100px;
    background-color: green;
  }

  #darkgreen {
    left: 200px;
    top: 200px;
    width: 100px;
    height: 100px;
    background-color: darkgreen;
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
