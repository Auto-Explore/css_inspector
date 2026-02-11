# css/css-ui/inheritance.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/inheritance.html"
}
```

## style[0]

```css

#reference {
  border-style: dotted; /* Avoid border-top-width computed style 0 */
  border-top-width: medium;
  color: lime;
  background: blue;
}

#container, #target {
  outline-style: dotted; /* Avoid outline-width computed style 0 */
  color: lime;
  background: blue;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
