# css/css-box/margin-trim/block-container-non-adjoining-item.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/block-container-non-adjoining-item.html"
}
```

## style[0]

```css

.container {
    background-color: green;
    width: min-content;
    margin-bottom: 100px;
    padding-bottom: 50px;
    background-clip: padding-box;
    margin-trim: block;
}
.child {
    margin-top: 50px;
    margin-bottom: 200px;
    width: 100px;
    height: 50px;
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
