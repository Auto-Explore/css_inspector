# css/css-page/margin-boxes/content-008-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/margin-boxes/content-008-print.html"
}
```

## style[0]

```css

  @page {
    margin: 4em;
    counter-increment: page 2;

    @bottom-center {
      text-align: left;
      vertical-align: top;
      content: counter(page);
    }
  }
  @page :first {
    @top-center {
      text-align: left;
      vertical-align: top;
      content: "There are " counter(pages) " pages";
    }
  }
  body {
    margin: 0;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
