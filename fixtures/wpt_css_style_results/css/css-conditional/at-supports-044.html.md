# css/css-conditional/at-supports-044.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-044.html"
}
```

## style[0]

```css

  div {
    background: green;
    height: 20px;
    width: 100px;
  }

  body {
    --badcolor: red;
    --goodcolor: green;
  }
  div {
    background: var(--badcolor);
  }
  @supports (--foo: whatever) {
    .test1 { background: green; }
  }

  @supports (--foo: whatever !important) {
    .test2 { background: var(--goodcolor); }
  }

  .test3 { background: green; }
  @supports (--foo: whatever !bogus) {
    .test3 { background: red; }
  }

  @supports (color: var(--anything) invalid-value) {
    .test4 { background: green; }
  }

  .test5 { background: red; }
  @supports not (--goodcolor: green) {
    .test5 { background: green; }
  }
  .test5 { background: var(--goodcolor) }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
