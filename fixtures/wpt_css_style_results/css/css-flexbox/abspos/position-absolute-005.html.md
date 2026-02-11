# css/css-flexbox/abspos/position-absolute-005.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/abspos/position-absolute-005.html"
}
```

## style[0]

```css

  body {
    width: 400px;
    height: 300px;
  }

  .flexbox {
    display: flex;
  }

  .column {
    flex-direction: column;
  }

  .flex11a {
    flex: 1 1 auto;
  }

  .root {
    height: 100px;
    overflow-y: auto;
    position: relative;
  }

  #abspos {
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
    height: 10px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
