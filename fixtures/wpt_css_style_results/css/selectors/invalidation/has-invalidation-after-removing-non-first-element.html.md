# css/selectors/invalidation/has-invalidation-after-removing-non-first-element.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/has-invalidation-after-removing-non-first-element.html"
}
```

## style[0]

```css

div, main { color: grey }
#subject:has(descendant) { color: red }
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
