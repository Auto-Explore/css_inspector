# css/css-box/margin-trim/block-container-block-end-002.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/block-container-block-end-002.html"
}
```

## style[0]

```css

.container {
    width: min-content;
    margin-trim: block-end;
    background-color: green;
}
.outer {
    width: 100px;
    height: 40px;
    background-color: green;
}
.first {
    width: 100px;
    margin-bottom: 25px;
    height: 25px;
}
.second {
    margin-bottom: 100px;
    width: 100px;
    height: 10px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “margin-trim”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
