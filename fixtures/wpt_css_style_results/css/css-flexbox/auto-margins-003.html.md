# css/css-flexbox/auto-margins-003.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/auto-margins-003.html"
}
```

## style[0]

```css

    .flexbox {
      border: 1px solid black;
      width: 400px;
      height: 200px;
    }
    .item1 {
      margin: 0 auto;
      background: lightblue;
    }
    .item1v {
      margin: auto 0;
      background: lightblue;
    }
    .item2 {
      background: lime;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
