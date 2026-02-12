# css/css-contain/contain-layout-overflow-002.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-layout-overflow-002.html"
}
```

## style[0]

```css

  .contain {
    contain: layout;
  }
  .float { float: left; }
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
  .inner-lg {
    height: 200px;
    width: 200px;
    background: lightblue;
  }
  .pass {
    background: green;
  }
  .fail {
    background: red;
  }
  .border {
    border: 5px solid green;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
