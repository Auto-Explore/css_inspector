# css/css-images/gradient-nan-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient-nan-crash.html"
}
```

## style[0]

```css
body { background: linear-gradient(black calc(0% * (1e39 - 1e39)), black 0%); }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
