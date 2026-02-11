# css/css-contain/contain-layout-overflow-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-overflow-002-ref.html"
}
```

## style[0]

```css

  .outer {
    height: 100px;
    width: 100px;
  }
  .auto {
    overflow: auto;
  }
  .inner-sm {
    height: 50px;
    width: 50px;
    background: lightblue;
  }
  .inner-lg-1 {
    height: 95px;
    width: 95px;
    float:left;
    background: lightblue;
  }
  .inner-lg-2 {
    height: 200px;
    width: 200px;
    float:left;
  }
  .pass {
    background: green;
  }
  .border {
    border: 5px solid green;
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
