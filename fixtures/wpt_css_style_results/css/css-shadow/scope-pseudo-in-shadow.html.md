# css/css-shadow/scope-pseudo-in-shadow.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/scope-pseudo-in-shadow.html"
}
```

## style[0]

```css

      div {
        background-color: green;
      }
      :scope div {
        background-color: red;
      }
      :scope > div {
        background-color: red;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
