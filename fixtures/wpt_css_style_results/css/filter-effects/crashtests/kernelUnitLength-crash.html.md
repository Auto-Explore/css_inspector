# css/filter-effects/crashtests/kernelUnitLength-crash.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/crashtests/kernelUnitLength-crash.html"
}
```

## style[0]

```css

div {
  filter: url(#a);
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
