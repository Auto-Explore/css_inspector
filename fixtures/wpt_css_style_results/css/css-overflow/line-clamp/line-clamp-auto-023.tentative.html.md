# css/css-overflow/line-clamp/line-clamp-auto-023.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-auto-023.tentative.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: auto;
  max-height: 7lh;
  font: 16px / 32px serif;
  background-color: yellow;
}
.clamp > div {
  /*
   * The total border across the top and bottom of one element is 16px.
   * The top and bottom borders across two elements add up to 32px = 1lh.
   */
  padding: 6px;
  border: 2px solid black;
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
