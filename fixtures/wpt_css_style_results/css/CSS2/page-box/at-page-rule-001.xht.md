# css/CSS2/page-box/at-page-rule-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/page-box/at-page-rule-001.xht"
}
```

## style[0]

```css

    @page {
      @import "support/import-red";
      margin-left: 50%;
      @media print { }
      margin-top: 50%;
    }
    @page {
      @media print { }
      margin-top: 0;
    }
    @page {
      margin-top: 0
      @media print {}
    }
    @page {
      margin-top: 0
      @media print;
    }
    @page {
      @import "support/import-red"
      margin-top: 0;
    }
    html, body, p {
      margin: 0;
      padding: 0;
    }
    p {
      border: solid blue;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
