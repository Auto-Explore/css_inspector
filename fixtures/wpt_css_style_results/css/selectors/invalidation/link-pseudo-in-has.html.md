# css/selectors/invalidation/link-pseudo-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/link-pseudo-in-has.html"
}
```

## style[0]

```css

  #parent { color: blue; }
  #grandparent { color: blue; }
  #parent:has(> :not(:link)) { color: grey; }
  #parent:has(> :link) { color: green; }
  #parent:has(> :visited) { color: red; }
  #grandparent:has(:not(:any-link)) { color: grey; }
  #grandparent:has(:any-link) { color: green; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
