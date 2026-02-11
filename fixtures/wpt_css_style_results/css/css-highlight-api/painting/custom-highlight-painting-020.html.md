# css/css-highlight-api/painting/custom-highlight-painting-020.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/custom-highlight-painting-020.html"
}
```

## style[0]

```css

  @font-face {
    font-family: 'mplus';
    src: url('../../../fonts/mplus-1p-regular.woff');
  }
  ::highlight(foo) {
    color:green;
    text-decoration: blue 2px line-through;
  }
  div {
      font-size: 7em;
      font-family: mplus, serif;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-decoration”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
