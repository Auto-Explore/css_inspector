# css/css-conditional/container-queries/container-units-selection-pseudo.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/container-units-selection-pseudo.html"
}
```

## style[0]

```css

  #outer, #inner {
    container-type: size;
  }
  #outer {
    width: 200px;
    height: 200px;
    border: 1px solid black;
  }
  #inner {
    width: 100px;
    height: 100px;
    background-color: skyblue;
  }
  #inner::selection {
    background-color: seagreen;
    text-decoration: underline solid;
    text-underline-offset: 10cqh;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
