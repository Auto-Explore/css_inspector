# css/css-text/word-break/auto-phrase/reference/word-break-auto-phrase-fallback-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text/word-break/auto-phrase/reference/word-break-auto-phrase-fallback-001-ref.html"
}
```

## style[0]

```css

div {
  font-size: 2em;
  border: solid orange;
  margin: 5px;
  width: min-content; /*not 0, to avoid falling into overflow fallback behavior */
  word-break: normal;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
