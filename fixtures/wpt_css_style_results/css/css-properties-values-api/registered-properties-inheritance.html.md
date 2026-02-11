# css/css-properties-values-api/registered-properties-inheritance.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/registered-properties-inheritance.html"
}
```

## style[0]

```css

#outer {
    --inherited-length-1: 10px;
    --inherited-length-2: var(--non-inherited-length-1);
    --inherited-length-3: 30px;
    --non-inherited-length-1: 22px;
    --non-inherited-length-3: calc(var(--non-inherited-length-2) * 10);
}

#inner {
    --inherited-length-3: 15px;
    --non-inherited-length-1: 40px;
    --non-inherited-length-2: 90px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
