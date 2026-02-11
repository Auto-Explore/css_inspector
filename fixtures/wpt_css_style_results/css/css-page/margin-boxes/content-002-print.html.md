# css/css-page/margin-boxes/content-002-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/margin-boxes/content-002-print.html"
}
```

## style[0]

```css

  @page {
    margin: 4em;

    @top-left {
      text-align: left;
      vertical-align: top;
      content: open-quote "Trøndere gråter når " open-quote "Vinsjan på kaia" close-quote " blir deklamert" close-quote;
    }

    @bottom-left {
      quotes: "[" "]" "{" "}";
      text-align: left;
      vertical-align: top;
      content: open-quote "Trøndere gråter når " open-quote "Vinsjan på kaia" close-quote " blir deklamert" close-quote;
    }
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “quotes”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
