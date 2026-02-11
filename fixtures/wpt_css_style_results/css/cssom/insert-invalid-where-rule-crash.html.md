# css/cssom/insert-invalid-where-rule-crash.html

```json
{
  "format_version": 3,
  "file": "css/cssom/insert-invalid-where-rule-crash.html"
}
```

## style[0]

```css
.x, :where("something invalid") {} 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
