# css/css-variables/variable-substitution-replaced-size.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-substitution-replaced-size.html"
}
```

## style[0]

```css

        .testcases > *
        {
            border: none;
            --w: 10px;
            --h: 10px;
            width: calc(var(--w) + 20px);
            height: calc(var(--h) + 20px);
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
