# css/selectors/has-display-none-checked.html

```json
{
  "format_version": 3,
  "file": "css/selectors/has-display-none-checked.html"
}
```

## style[0]

```css

body:has(input:checked) #fail {
  display: none;
}
body:not(:has(input:checked)) #pass {
  display: none;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
