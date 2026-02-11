# css/selectors/scope-without-scoping.html

```json
{
  "format_version": 3,
  "file": "css/selectors/scope-without-scoping.html"
}
```

## style[0]

```css

  div {
    width: 100px;
    height: 100px;
    background: red;
  }
  :scope > body > div {
    background: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
