# css/css-overflow/line-clamp/line-clamp-021.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-021.html"
}
```

## style[0]

```css

#scrollContainer {
  overflow: scroll;
  font: 16px / 32px serif;
  height: 3lh;
  border: 1px solid black;
}
.clamp {
  line-clamp: 4;
  padding: 4px;
  background-color: yellow;
}
.pre {
  white-space: pre;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “line-clamp”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
