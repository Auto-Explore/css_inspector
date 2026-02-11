# css/css-values/ex-unit-001.html

```json
{
  "format_version": 3,
  "file": "css/css-values/ex-unit-001.html"
}
```

## style[0]

```css

  @font-face {
    font-family: foo;
    src: url('/fonts/Ahem.ttf');
  }

  @font-face {
    font-family: foo;
    font-weight: 900;
    /* A font with significantly different ex-height metric than Aham. */
    src: url('/fonts/noto/noto-sans-v8-latin-regular.woff');
  }

  div {
    font-family: foo, sans-serif;
    width: 10ex;
    height: 20px;
    background: blue;
    margin: 20px;
    font-size: 20px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
