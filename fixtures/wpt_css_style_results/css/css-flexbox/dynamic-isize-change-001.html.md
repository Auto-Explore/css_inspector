# css/css-flexbox/dynamic-isize-change-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/dynamic-isize-change-001.html"
}
```

## style[0]

```css

  #container {
    border: 1px solid black;
    font: 30px monospace;
    width: 200px;
  }
  .inline-flex {
    display: inline-flex;
    width: 100px;
    border: 1px solid gray;
  }

  .small {
    display: flex;
    align-items: flex-end;
    font: 8px monospace;
    background: pink;
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
