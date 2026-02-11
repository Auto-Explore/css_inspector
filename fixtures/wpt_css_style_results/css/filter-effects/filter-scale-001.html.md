# css/filter-effects/filter-scale-001.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-scale-001.html"
}
```

## style[0]

```css

  #outer {
    width: 15px;
    height: 15px;
    transform: scale(10);
    transform-origin: 0 0;
  }
  #inner {
    filter: url(#blur);
    width: 15px;
    height: 15px;
    background: green;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
