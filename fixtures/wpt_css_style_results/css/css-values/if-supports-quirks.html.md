# css/css-values/if-supports-quirks.html

```json
{
  "format_version": 3,
  "file": "css/css-values/if-supports-quirks.html"
}
```

## style[0]

```css

    div {
      color: red;
      --x: if(supports(width: 30): true_value; else: false_value;)
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
