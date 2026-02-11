# css/css-sizing/aspect-ratio/flex-aspect-ratio-044.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/aspect-ratio/flex-aspect-ratio-044.html"
}
```

## style[0]

```css

#container {
  display: flex;
  flex-direction: column;
  inline-size: 100px;
  aspect-ratio: 2 / 1;
  background: green;
  max-block-size: 100px;
}
#item {
  flex: 0 0 200px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
