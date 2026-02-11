# css/css-page/margin-boxes/content-009-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/margin-boxes/content-009-print.html"
}
```

## style[0]

```css

  @page {
    margin: 4em;

    @bottom-center {
      text-align: left;
      vertical-align: top;
      content: counter(page);
    }
  }
  @page :left {
    counter-increment: page 3;
  }
  @page :right {
    counter-increment: page 0;
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
