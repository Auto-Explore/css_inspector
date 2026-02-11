# css/cssom/getComputedStyle-display-none-002.html

```json
{
  "format_version": 3,
  "file": "css/cssom/getComputedStyle-display-none-002.html"
}
```

## style[0]

```css

#undisplayed {
  display: none;
  color: red;
}
.sibling + #undisplayed {
  color: green;
}

.sibling + #undisplayed > div {
  color: yellow;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
