# css/selectors/invalidation/has-side-effect.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-side-effect.html"
}
```

## style[0]

```css

div, main { color: grey }
#subject:has(+ #next_sibling) { color: red; }
#prev_sibling:has(+ #subject + #next_sibling) { color: green; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
