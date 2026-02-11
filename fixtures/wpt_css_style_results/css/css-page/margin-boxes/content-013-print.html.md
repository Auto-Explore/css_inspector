# css/css-page/margin-boxes/content-013-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/margin-boxes/content-013-print.html"
}
```

## style[0]

```css

  html {
    counter-reset: pages 10;
  }
  @page {
    margin: 4em;
    counter-increment: pages 2;

    @top-center {
      text-align: left;
      vertical-align: top;
      counter-increment: pages;
      content: counters(pages, ".");
    }

    @bottom-center {
      text-align: left;
      vertical-align: top;
      counter-reset: pages 2;
      counter-increment: pages;
      content: counters(pages, ".");
    }
  }
  body {
    margin: 0;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “content”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
