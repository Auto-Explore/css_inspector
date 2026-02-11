# css/css-layout-api/position-fragment/ref.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/position-fragment/ref.html"
}
```

## style[0]

```css

.result {
  position: relative;
  background: green;
  width: 100px;
  height: 100px;
}

.result-child-1 {
  background: rebeccapurple;
  width: 10px;
  height: 20px;

  position: absolute;
  top: 25px;
  left: 5px;
}

.result-child-2 {
  background: rebeccapurple;
  width: 15px;
  height: 25px;

  position: absolute;
  top: 60px;
  left: 50px;
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
