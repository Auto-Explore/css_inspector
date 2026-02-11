# css/css-text/white-space/textarea-pre-wrap-014.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/textarea-pre-wrap-014.html"
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
  white-space: pre-wrap;
  color: white;

  background: linear-gradient(red, red) 3ch 0/2ch 1ch no-repeat;

  width: 4ch;
  text-align: justify;
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
