# css/css-pseudo/highlight-cascade/highlight-pseudos-computed-search-text.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-pseudos-computed-search-text.tentative.html"
}
```

## style[0]

```css

  #target::search-text {
    background-color: blue;
    color: lime;
  }
  #target::search-text:not(:current) {
    background-color: green;
  }
  #target::search-text:current {
    /* FAIL if this matches */
    background-color: red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
