# css/css-overflow/line-clamp/line-clamp-with-fixed-pos-017.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-with-fixed-pos-017.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: 4;
  font: 16px / 32px serif;
  padding: 0 4px;
  white-space: pre;
  background-color: yellow;
}
.transformed {
  transform: scale(1); /* Makes it a fixed-pos containing block */
}
.fixed {
  position: fixed;
  right: 0;
  width: 100px;
  /* No height! */
  margin: 4px;
  white-space: normal;
  background-color: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
