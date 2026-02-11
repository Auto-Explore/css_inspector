# css/css-page/margin-boxes/content-006-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/margin-boxes/content-006-print.html"
}
```

## style[0]

```css

  @page {
    margin: 4em;

    @top-center {
      text-align: left;
      vertical-align: top;
      content: "Page " counter(page) " of " counter(pages);
    }
    @bottom-center {
      text-align: left;
      vertical-align: top;
      content: "Page " counter(page) " of " counter(pages);
    }
  }
  @page :first {
    @top-center { content: none; }
    @bottom-center { content: none; }
  }
  body {
    margin: 0;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
