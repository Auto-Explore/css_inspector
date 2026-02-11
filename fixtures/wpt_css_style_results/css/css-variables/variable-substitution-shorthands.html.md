# css/css-variables/variable-substitution-shorthands.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-substitution-shorthands.html"
}
```

## style[0]

```css

        #target7parent > #target7 {
            margin: var(--invalid); /* invalid at compute time */
        }

        #target7 {
            margin: 77px;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
