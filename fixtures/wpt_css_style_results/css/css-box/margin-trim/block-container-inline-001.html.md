# css/css-box/margin-trim/block-container-inline-001.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/block-container-inline-001.html"
}
```

## style[0]

```css

.container {
    width: 100px;
    margin-trim: inline;
}

.child {
    background-color: green;
    width: 100px;
    height: 50px;
    margin-inline: 50px;
    position: relative;
    left: -50px;
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
