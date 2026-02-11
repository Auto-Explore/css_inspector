# css/css-page/margin-boxes/content-010-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/margin-boxes/content-010-print.html"
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

    @bottom-center {
      text-align: left;
      vertical-align: top;
      content: counter(foo);
    }
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
