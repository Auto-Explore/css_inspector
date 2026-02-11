# css/css-text/letter-spacing/reference/letter-spacing-205-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text/letter-spacing/reference/letter-spacing-205-ref.html"
}
```

## style[0]

```css

  @import "/fonts/ahem.css";
  .contain {
    font: 20px/1 Ahem;
    border: solid blue;
    margin: 1em;
    width: max-content; }
  .ls1 {
    letter-spacing: 1em;
  }
  span {
    border: solid orange;
  }
  .control p {
    white-space: pre-wrap;
  }
  p {
    letter-spacing: 0;
    margin: 0;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
