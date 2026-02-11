# css/selectors/attribute-selectors/style-attribute-selector.html

```json
{
  "format_version": 3,
  "file": "css/selectors/attribute-selectors/style-attribute-selector.html"
}
```

## style[0]

```css

  #container { font-size: 16px; color: black; }
  .test[style] { color: green }
  .test[style=""] { font-size: 100px }
  .test[style*="text-decoration"] { background-color: lime }
  .test[style] + #sibling { color: green; }
  .test[style*="text-decoration"] + #sibling { background-color: lime; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
