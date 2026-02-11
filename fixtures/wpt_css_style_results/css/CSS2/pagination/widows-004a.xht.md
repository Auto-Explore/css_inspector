# css/CSS2/pagination/widows-004a.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/pagination/widows-004a.xht"
}
```

## style[0]

```css

  html, body {
    height: 100%;
    line-height: 1;
    font-size: 20px;
  }
  * {
    margin:0;
    padding:0;
  }
  div.spacer {
    height:50%;
  }
  div.backup {
    margin-top:-5em;
  }

  .test {
    color: blue;
    width: 1em;
    widows: 5; /* valid */
    widows: 2.0; /* invalid: real and not integer */
    widows: 2em; /* invalid: length and not integer */
    widows: -2; /* invalid: negative integer */
    widows: 0; /* invalid: zero is not a positive integer */
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
