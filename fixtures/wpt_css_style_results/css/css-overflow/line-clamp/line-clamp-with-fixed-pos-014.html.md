# css/css-overflow/line-clamp/line-clamp-with-fixed-pos-014.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/line-clamp-with-fixed-pos-014.html"
}
```

## style[0]

```css

.clamp {
  line-clamp: 4;
  font: 16px / 32px serif;
  padding: 0 4px;
  background-color: yellow;
}
.transformed {
  transform: scale(1); /* Makes it a fixed-pos containing block */
}
.fixed {
  position: fixed;
  top: 0;
  left: 0;
  width: 20px;
  height: 20px;
  background-color: skyblue;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
