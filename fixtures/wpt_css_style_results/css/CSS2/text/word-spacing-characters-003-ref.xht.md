# css/CSS2/text/word-spacing-characters-003-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/text/word-spacing-characters-003-ref.xht"
}
```

## style[0]

```css

    div {
      margin: 1em;
      font-family: monospace;
      page-break-inside: avoid;
    }
    div p {
      margin: 0;
    }

    .control span {
      background: blue;
      color: blue;
    }
    .test span {
      background: orange;
      color: orange;
    }
    .ws-pre p {
      white-space: pre;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
