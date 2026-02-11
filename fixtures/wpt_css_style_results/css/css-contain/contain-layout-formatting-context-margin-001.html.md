# css/css-contain/contain-layout-formatting-context-margin-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-formatting-context-margin-001.html"
}
```

## style[0]

```css

  #a {
      contain:layout;
      background: blue;
      margin: 10px;
      width: 50px;
      height: 50px;
  }
  #b {
      width: 50px;
      height: 40px;
      background: green;
      margin-top: 10px;
  }
  #c {
      background: lightblue;
      width: 50px;
      height: 10px;
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
