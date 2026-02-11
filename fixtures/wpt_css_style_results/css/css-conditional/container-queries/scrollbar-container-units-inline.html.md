# css/css-conditional/container-queries/scrollbar-container-units-inline.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scrollbar-container-units-inline.html"
}
```

## style[0]

```css

.container {
  display: inline-block;
  border: solid 3px cornflowerblue;
  width: 100px;
  height: 100px;
  container-type: inline-size;
}
div > div {
  box-sizing: border-box;
  width: 100cqw;
  height: 200px;
  border: solid 10px orange;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
