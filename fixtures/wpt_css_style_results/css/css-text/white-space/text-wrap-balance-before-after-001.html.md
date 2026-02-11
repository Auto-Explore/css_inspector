# css/css-text/white-space/text-wrap-balance-before-after-001.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/text-wrap-balance-before-after-001.html"
}
```

## style[0]

```css

.container {
  font: 16px monospace;
  margin: 1em;
  width: 60ch;
}

.text1, .text2 {
  text-wrap: balance;
  margin-bottom: 1em;
  outline: 1px dashed gray;
}

/* :before and :after pseudos with display:block and no content should not affect
   the layout of the paragraph content */
.text2:before, .text2:after {
  content: "";
  display: block;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
