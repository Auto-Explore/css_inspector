# css/css-variables/variable-recalc-with-initial.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-recalc-with-initial.html"
}
```

## style[0]

```css

    .a {
      --color: red;
    }
    .b {
      --color: initial;
    }
    .c {
      color: var(--color, green);
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
