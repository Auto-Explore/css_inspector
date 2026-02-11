# css/css-tables/table-caption-after-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/table-caption-after-crash.html"
}
```

## style[0]

```css

*::after, .a:only-of-type {
  content: counters(ct1, '.', disc);
  display: table-caption;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
