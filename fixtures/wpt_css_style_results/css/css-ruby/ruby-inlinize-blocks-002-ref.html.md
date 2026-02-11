# css/css-ruby/ruby-inlinize-blocks-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-ruby/ruby-inlinize-blocks-002-ref.html"
}
```

## style[0]

```css

    body {
      /* Use a sans-serif font to avoid fuzzy pixels where e.g. the
         letter "a" bottom-right serif might overlap the table border: */
      font: 16px sans-serif;
    }
    .block, table, .flex {
      background-color: yellow;
      width: 100px; height: 30px;
      border: 1px solid blue;
    }
    .block { display: inline-block; }
    table { display: inline-table; border-collapse: collapse; }
    td { border: 3px solid red; }
    .flex { display: inline-flex; }
    .flex-item { flex: auto; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
