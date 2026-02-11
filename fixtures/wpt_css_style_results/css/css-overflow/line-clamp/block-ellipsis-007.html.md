# css/css-overflow/line-clamp/block-ellipsis-007.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/block-ellipsis-007.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: 1;
  color: teal;
}
.clamp::first-line {
  color: purple;
  font-weight: bold;
  font-style: italic;
  font-size: 1.5em;
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
