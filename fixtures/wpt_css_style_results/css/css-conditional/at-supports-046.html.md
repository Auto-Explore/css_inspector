# css/css-conditional/at-supports-046.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/at-supports-046.html"
}
```

## style[0]

```css

  div {
    background: green;
    height: 10px;
    width: 100px;
  }

  div {
    background: red;
  }

  @supports not unknown() {
    .test1.bare { background: green; }
  }
  @supports (not (unknown())) {
    .test1.wrapped { background: green; }
  }

  @supports not unknown(with stuff) {
    .test2.bare { background: green; }
  }
  @supports (not (unknown(with stuff))) {
    .test2.wrapped { background: green; }
  }

  @supports not unknown(!@#% { ... } more() @stuff [ ]) {
    .test3.bare { background: green; }
  }
  @supports (not (unknown(!@#% { ... } more() @stuff [ ]))) {
    .test3.wrapped { background: green; }
  }

  .test4, .test5 { background: green; }

  @supports unknown() {
    .test4.bare { background: red; }
  }
  @supports (unknown()) {
    .test4.wrapped { background: red; }
  }

  @supports unknown(with stuff) {
    .test5.bare { background: red; }
  }
  @supports (unknown(with stuff)) {
    .test5.wrapped { background: red; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
