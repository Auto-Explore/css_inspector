# css/selectors/invalidation/defined-in-has.html

```json
{
  "format_version": 3,
  "file": "css/selectors/invalidation/defined-in-has.html"
}
```

## style[0]

```css

  #subject {
    background-color: red;
    width: 100px;
    height: 100px;
  }
  #subject:has(:defined) {
    background-color: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
