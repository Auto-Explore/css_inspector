# css/css-overflow/line-clamp/webkit-line-clamp-010.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/webkit-line-clamp-010.html"
}
```

## style[0]

```css

.clamp {
  display: -webkit-box;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 5;
  font: 16px / 32px serif;
  white-space: pre;
  padding: 0 4px;
  background-color: yellow;
  overflow: hidden; /* can be removed once implementations update their old -webkit-line-clamp implementations */
}
.child {
  font: 24px / 48px serif;
  color: blue;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
