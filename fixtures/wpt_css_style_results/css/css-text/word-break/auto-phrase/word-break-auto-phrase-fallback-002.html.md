# css/css-text/word-break/auto-phrase/word-break-auto-phrase-fallback-002.html

```json
{
  "format_version": 3,
  "file": "css/css-text/word-break/auto-phrase/word-break-auto-phrase-fallback-002.html"
}
```

## style[0]

```css

div {
  font-size: 2em;
  border: solid blue;
  margin: 5px;
  width: min-content; /*not 0, to avoid falling into overflow fallback behavior */
  word-break: normal;
}
#test {
  word-break: auto-phrase;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
