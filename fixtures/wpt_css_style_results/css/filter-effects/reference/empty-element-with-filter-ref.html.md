# css/filter-effects/reference/empty-element-with-filter-ref.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/reference/empty-element-with-filter-ref.html"
}
```

## style[0]

```css

  .turbulent {
    width: 160px;
    height: 90px;
    margin: 10px;
    filter: url(#turbulence);
  }

  .visibleContent {
    background-color: red;
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
