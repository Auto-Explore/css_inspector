# css/css-break/table/repeated-section/special-elements-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-break/table/repeated-section/special-elements-crash.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }
  .table {
    display: table;
    width: 100%;
  }
  .header {
    display: table-header-group;
    break-inside: avoid;
  }
  .filler {
    display: table-row;
    break-inside: avoid;
    height: 2600px;
  }
  .header > * {
    /* Don't make stuff too tall. We want everything (in the header) to be
       within the viewport. */
    height: 10px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
