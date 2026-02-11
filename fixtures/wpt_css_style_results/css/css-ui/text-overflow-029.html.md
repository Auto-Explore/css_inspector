# css/css-ui/text-overflow-029.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/text-overflow-029.html"
}
```

## style[0]

```css

div {
  font: 20px monospace;
  width: 12.3ch; /* slightly more than 12ch because in some browsers (safari) the ellipsis is slightly large than other characters, even in monospace fonts. */
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
