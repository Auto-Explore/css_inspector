# css/css-overflow/line-clamp/line-clamp-auto-020.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-auto-020.tentative.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: auto;
  max-height: 6lh;
  font: 16px / 32px serif;
  white-space: pre;
  background-color: yellow;
}
.clamp > div {
  /* The total block-size bpm across the top and bottom is 2*(15+2) = 34px, greater than the line-height */
  padding: 15px;
  border: 2px solid black;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
