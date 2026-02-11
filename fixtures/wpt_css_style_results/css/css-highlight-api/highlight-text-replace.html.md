# css/css-highlight-api/highlight-text-replace.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/highlight-text-replace.html"
}
```

## style[0]

```css

        :root {
            /* This reduces the likelihood that a highlight background-area
               will overlap with the previous glyph.  That overlap is worth
               avoiding, because the previous glyph's overlapping part will get
               clipped in the reference case, but might not in the testcase. */
            font-family: monospace;
        }
        ::highlight(example-highlight) {
            background-color: yellow;
            color:green;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
