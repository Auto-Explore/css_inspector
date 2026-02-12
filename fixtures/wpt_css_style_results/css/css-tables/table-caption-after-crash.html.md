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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
