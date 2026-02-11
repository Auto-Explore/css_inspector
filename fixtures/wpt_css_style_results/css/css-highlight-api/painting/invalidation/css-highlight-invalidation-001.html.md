# css/css-highlight-api/painting/invalidation/css-highlight-invalidation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/painting/invalidation/css-highlight-invalidation-001.html"
}
```

## style[0]

```css

  ::highlight(example) {
    text-decoration: wavy underline overline green 5px;
    text-underline-offset: 20px;
  }
  div {
    border: solid 1px black;
    padding: 50px;
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
