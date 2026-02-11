# css/css-env/fallback-nested-var.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-env/fallback-nested-var.tentative.html"
}
```

## style[0]

```css

      body {
        --main-bg-color: rgb(0, 128, 0);
        background-color: env(test, var(--main-bg-color));
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
