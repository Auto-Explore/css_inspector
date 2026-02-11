# css/filter-effects/crashtests/multiple-references-id-crash-002.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/crashtests/multiple-references-id-crash-002.html"
}
```

## style[0]

```css

#move, #obj, #bdo {
  filter: url(#target);
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
