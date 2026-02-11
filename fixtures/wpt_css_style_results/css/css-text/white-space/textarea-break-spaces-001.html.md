# css/css-text/white-space/textarea-break-spaces-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/textarea-break-spaces-001.html"
}
```

## style[0]

```css

textarea {
  word-wrap: initial; /*deprecated alias*/
  overflow-wrap: initial;
  line-break: initial;
  word-break: initial;
  margin: 0;
  padding: 0;
  border: none;
  outline: none;
  resize: none;
  overflow: hidden; /* I don't want scrollbars, and overflow:visible isn't typically supported on textarea */

  font-size: 20px;
  font-family: Ahem;
  line-height: 1em;
  white-space: break-spaces;
  color: green;

  background: linear-gradient(red, red) 1ch 0/2ch 2ch no-repeat;

  width: 4ch;
  margin-left: -1ch;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
