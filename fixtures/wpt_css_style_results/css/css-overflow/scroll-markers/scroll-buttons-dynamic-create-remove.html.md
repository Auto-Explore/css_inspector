# css/css-overflow/scroll-markers/scroll-buttons-dynamic-create-remove.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-buttons-dynamic-create-remove.html"
}
```

## style[0]

```css

#carousel {
  overflow: hidden;
  width: 100px;
  height: 100px;
}

#carousel div {
  background: green;
  width: 100px;
  height: 100px;
}

.scroll-buttons::scroll-button(right) {
  content: "r";
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
