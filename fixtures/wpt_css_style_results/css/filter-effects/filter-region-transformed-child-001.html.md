# css/filter-effects/filter-region-transformed-child-001.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-region-transformed-child-001.html"
}
```

## style[0]

```css

#parent {
    display: inline-block;
    filter: url(#filter);
}
#child {
    background-color: gray;
    width: 50px;
    height: 50px;
    transform: translate(25px, 25px) scale(2);
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
