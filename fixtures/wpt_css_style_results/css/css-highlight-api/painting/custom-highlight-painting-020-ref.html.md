# css/css-highlight-api/painting/custom-highlight-painting-020-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/custom-highlight-painting-020-ref.html"
}
```

## style[0]

```css

  @font-face {
    font-family: 'mplus';
    src: url('../../../fonts/mplus-1p-regular.woff');
  }
  div {
      font-size: 7em;
      font-family: mplus, sans-serif;
  }
  ::selection {
    color:green;
    text-decoration: blue 2px line-through;
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
