# css/css-box/margin-trim/block-container-block-002.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/block-container-block-002.html"
}
```

## style[0]

```css

.container {
    background-color: green;
    width: min-content;
    margin-trim: block;
}
.first {
    margin-top: 50px;
    height: 25px;
}
.second {
    width: 100px;
    height: 10px;
    margin-bottom: 15px;
}
.third {
    width: 100px;
    height: 25px;
    margin-bottom: 50px;
}
.outer {
    width: 100px;
    height: 25px;
    background-color: green;
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
