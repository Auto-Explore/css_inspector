# css/filter-effects/backdrop-filter-feimage-crash.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/backdrop-filter-feimage-crash.html"
}
```

## style[0]

```css

* {
  backdrop-filter: url(#a);
  filter: saturate(60%);
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
