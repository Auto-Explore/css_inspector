# css/css-env/env-in-custom-properties.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-env/env-in-custom-properties.tentative.html"
}
```

## style[0]

```css

      #parent {
        --var1: inherited;
      }
      #child {
        --my-width: env(test, 100px);
        width: var(--my-width);
        --var1: env(nonexistent);
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
