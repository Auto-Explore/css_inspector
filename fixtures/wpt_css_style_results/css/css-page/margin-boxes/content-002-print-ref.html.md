# css/css-page/margin-boxes/content-002-print-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-page/margin-boxes/content-002-print-ref.html"
}
```

## style[0]

```css

  @page {
    margin: 0;
  }
  body {
    margin: 0;
  }
  .content::before {
    content: open-quote "Trøndere gråter når " open-quote "Vinsjan på kaia" close-quote " blir deklamert" close-quote;
  }
  .bottom::before {
    quotes: "[" "]" "{" "}";
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “quotes”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
