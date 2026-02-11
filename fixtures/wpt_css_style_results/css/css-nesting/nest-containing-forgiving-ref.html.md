# css/css-nesting/nest-containing-forgiving-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/nest-containing-forgiving-ref.html"
}
```

## style[0]

```css

  .test {
    background-color: green;
    width: 100px;
    height: 100px;
    display: grid;
  }

  body * + * {
    margin-top: 8px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
