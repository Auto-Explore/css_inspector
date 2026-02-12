# css/css-nesting/nest-containing-forgiving.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/nest-containing-forgiving.html"
}
```

## style[0]

```css

  .test {
    background-color: red;
    width: 100px;
    height: 100px;
    display: grid;
  }

  .does-not-exist {
    :is(.test-1, !&) {
      background-color: green;
    }
  }

  .does-not-exist {
    :is(.test-2, :unknown(div,&)) {
      background-color: green;
    }
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
