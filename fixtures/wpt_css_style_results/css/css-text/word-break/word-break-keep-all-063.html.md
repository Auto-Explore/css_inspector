# css/css-text/word-break/word-break-keep-all-063.html

```json
{
  "format_version": 3,
  "file": "css/css-text/word-break/word-break-keep-all-063.html"
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
      */
      word-break: break-all;
      /*
      'word-break: break-all' applied on the
      wrapping block is necessary to verify
      that 'word-break: keep-all' is
      implemented
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
      word-break: keep-all;
    }

  div#reference
    {
      word-break: normal;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
