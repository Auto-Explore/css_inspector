# css/selectors/has-style-sharing-pseudo-003.html

```json
{
  "format_version": 3,
  "file": "css/selectors/has-style-sharing-pseudo-003.html"
}
```

## style[0]

```css

div {
  width: 20px;
  height: 20px;
}

:has(> .a) .b::after {
  font: 15px/1 ahem;
  content: 'x';
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
