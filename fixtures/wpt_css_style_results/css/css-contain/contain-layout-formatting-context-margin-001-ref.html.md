# css/css-contain/contain-layout-formatting-context-margin-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-formatting-context-margin-001-ref.html"
}
```

## style[0]

```css

  #a {
      background: blue;
      margin: 10px;
      width: 50px;
      height: 50px;
  }
  #b {
      width: 50px;
      height: 40px;
      background: green;
  }
  #b-padding {
      height: 10px;
  }
  #c {
    width: 50px;
    height: 10px;
    background: lightblue;
  }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
