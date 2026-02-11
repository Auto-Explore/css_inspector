# css/selectors/invalidation/any-link-pseudo.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/any-link-pseudo.html"
}
```

## style[0]

```css

      #link { background-color: red }
      #link:any-link { background-color: green }
      #link + div { color: pink }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
