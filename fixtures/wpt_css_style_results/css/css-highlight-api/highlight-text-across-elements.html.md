# css/css-highlight-api/highlight-text-across-elements.html

```json
{
  "format_version": 3,
  "file": "css/css-highlight-api/highlight-text-across-elements.html"
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
        ::highlight(example-highlight1) {
            background-color: yellow;
            color:green;
        }
        ::highlight(example-highlight2) {
            background-color: blue;
            color:red;
        }
        ::highlight(example-highlight3) {
            background-color: purple;
            color:pink;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
