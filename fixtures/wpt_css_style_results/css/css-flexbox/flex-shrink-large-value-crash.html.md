# css/css-flexbox/flex-shrink-large-value-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-shrink-large-value-crash.html"
}
```

## style[0]

```css

.flex {
    display: inline-flex;
    width: 40px;
    height: 40px;
}

.fractional {
    height: 50px;
    width: 50.5px;
    min-width: 50.5px;
}

.high-shrink {
    flex-shrink: 130000000000000;
    height: 40px;
    width: 40px;
    min-width: 40px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
