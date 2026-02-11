# css/css-nesting/conditional-properties-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/conditional-properties-ref.html"
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

  @media (min-width: 50px) {
    .test-5 > div {
      background-color: green;
    }
  }

  @supports (display: grid) {
    .test-10 {
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
