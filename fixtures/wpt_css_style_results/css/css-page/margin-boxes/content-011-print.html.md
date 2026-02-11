# css/css-page/margin-boxes/content-011-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/margin-boxes/content-011-print.html"
}
```

## style[0]

```css

  html {
    counter-reset: foo 10;
  }
  @page {
    margin: 4em;
    counter-increment: foo;

    @top-center {
      text-align: left;
      vertical-align: top;
      content: counters(foo, ".");
    }

    @bottom-center {
      text-align: left;
      vertical-align: top;
      counter-reset: foo 2;
      counter-increment: foo;
      content: counters(foo, ".");
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
