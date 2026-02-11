# css/css-overflow/overflow-body-propagation-008-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-body-propagation-008-ref.html"
}
```

## style[0]

```css

  html { overflow: auto hidden; }
  body {
    width: 30px;
    height: 30px;
    padding: 10px;
    margin-left: 100px;
    margin-top: 100px;
  }

  div {
    position: relative;
    top: -20px;
    left: -40px;
    background: blue;
    height: 10000px;
    width: 10000px;
    border-right-width: 20px;
    border-right-style: solid;
    border-right-color: red;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “overflow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
