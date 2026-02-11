# css/CSS2/page-box/page-grammar-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/page-box/page-grammar-001.xht"
}
```

## style[0]

```css

    @page {
      margin: 10%;    /* This makes the header big enough to hold the file name when printed on 4x6 in media. */
    }
    @page :right {
      margin-left: 50%;
    }
    @page :left {
      margin-right: 50%;
    }
    html {
      page-break-before: right;
    }
    p {
      padding: 0.5em;
      border: solid blue;
      page-break-after: always;
    }
   
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
