# css/css-nesting/conditional-rules-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/conditional-rules-ref.html"
}
```

## style[0]

```css

  .test {
    background-color: red;
    width: 30px;
    height: 30px;
    display: grid;
  }

  @media (min-width: 10px) {
    .test-5 > div {
      background-color: green;
    }
  }

  @media (min-width: 10px) {
    .test-6 > div {
      background-color: green;
    }
  }

  @supports (display: grid) {
    .test-10 {
      background-color: green;
    }
  }

  @layer {
    .test-11 {
      background-color: green !important;
    }
  }

  @scope (.test-12) {
    :scope {
      background-color: green;
    }
  }

  div {
    container-type: inline-size;
  }
  @container (width >= 0px) {
    .test-13 {
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
