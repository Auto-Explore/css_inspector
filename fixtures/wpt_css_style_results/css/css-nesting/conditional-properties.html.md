# css/css-nesting/conditional-properties.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/conditional-properties.html"
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

  .test-5 {
    @media (min-width: 50px) {
      background-color: green;
    }
  }

  .test-10 {
    @supports (display: grid) {
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
