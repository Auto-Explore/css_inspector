# css/css-text/word-break/word-break-break-all-062.html

```json
{
  "format_version": 3,
  "file": "css/css-text/word-break/word-break-break-all-062.html"
}
```

## style[0]

```css

  div
    {
      border: orange solid;
      font: 24px monospace;
      margin-bottom: 4px;
      width: 15ch;
      /*
      15 in 15ch is an entirely arbitrary number.
      The test only aims at checking if a word
      will break.
      */
    }

  div#ws-normal
    {
      white-space: normal;
    }

  div#ws-prewrap
    {
      white-space: pre-wrap;
    }

  div#ws-breakspaces
    {
      white-space: break-spaces;
    }

  div#ws-preline
    {
      white-space: pre-line;
    }

  span.test
    {
      word-break: break-all;
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
