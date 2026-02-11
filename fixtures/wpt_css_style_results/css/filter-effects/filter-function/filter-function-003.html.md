# css/filter-effects/filter-function/filter-function-003.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/filter-function/filter-function-003.html"
}
```

## style[0]

```css

.test {
    width: 100px;
    height: 100px;
    background: filter(url(resources/green-transparent-100x100.png), drop-shadow(50px 0 0 green)) top left no-repeat, red;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
