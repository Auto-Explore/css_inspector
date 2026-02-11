# css/css-pseudo/highlight-painting-currentcolor-004a-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-painting-currentcolor-004a-ref.html"
}
```

## style[0]

```css

  .a {
    color: yellow;
    background-color: blue;
  }
  .b {
    color: lime;
    background-color: blue;
  }
  .selection {
    color: currentColor;
    background-color: black;
    text-shadow: 0 2em red, 0 4em currentColor;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
