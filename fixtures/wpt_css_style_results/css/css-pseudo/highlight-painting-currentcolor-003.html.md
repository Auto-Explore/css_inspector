# css/css-pseudo/highlight-painting-currentcolor-003.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-painting-currentcolor-003.html"
}
```

## style[0]

```css

  ::highlight(a) {
    color: yellow;
    background-color: blue;
  }
  ::highlight(b) {
    color: lime;
    background-color: blue;
  }
  ::selection {
    color: currentColor;
    background-color: black;
    text-decoration: /* currentColor */ underline;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
