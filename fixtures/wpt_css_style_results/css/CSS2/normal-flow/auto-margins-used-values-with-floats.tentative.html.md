# css/CSS2/normal-flow/auto-margins-used-values-with-floats.tentative.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/auto-margins-used-values-with-floats.tentative.html"
}
```

## style[0]

```css

.container {
  display: flow-root;
  width: 100px;
  padding: 5px;
  box-sizing: border-box;
}
.box {
  display: flow-root;
  width: 40px;
  height: 10px;
  background: lime;
}
.float {
  float: right;
  width: 20px;
  height: 40px;
  background: cyan;
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
