# css/filter-effects/filtered-inline-applies-to-float.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filtered-inline-applies-to-float.html"
}
```

## style[0]

```css

span { filter: blur(2px); }
#float {
    float: left;
    width: 100px;
    height: 100px;
    background: green;
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
