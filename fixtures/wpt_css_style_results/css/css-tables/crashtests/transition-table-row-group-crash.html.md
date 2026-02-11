# css/css-tables/crashtests/transition-table-row-group-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-tables/crashtests/transition-table-row-group-crash.html"
}
```

## style[0]

```css

  optgroup {
    display: contents;
    writing-mode: vertical-lr;
  }
  option {
    display: table-row-group;
    transition: color 1s;
    --p: 2px;
    padding-inline: var(--p);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
