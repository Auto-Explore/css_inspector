# css/filter-effects/filter-external-002-test.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-external-002-test.html"
}
```

## style[0]

```css

    div {
        width: 200px;
        height: 200px;
        background: red;
        filter: url( support/filter-external-002-filter.svg#filter );
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
