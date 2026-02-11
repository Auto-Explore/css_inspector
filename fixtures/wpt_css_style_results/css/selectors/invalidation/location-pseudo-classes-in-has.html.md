# css/selectors/invalidation/location-pseudo-classes-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/location-pseudo-classes-in-has.html"
}
```

## style[0]

```css

  #parent1:has(:link) { color: green }
  #parent1:has(:visited) { color: yellowgreen }
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
