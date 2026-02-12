# css/css-page/margin-boxes/content-012-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/margin-boxes/content-012-print.html"
}
```

## style[0]

```css

  html {
    counter-reset: foo 10;
  }
  @page {
    margin: 4em;
    counter-reset: foo 5;
    counter-increment: foo;

    @top-left {
      width: 50%;
      text-align: left;
      vertical-align: top;
      content: counters(foo, ".");
    }

    @top-right {
      width: 50%;
      text-align: left;
      vertical-align: top;
      counter-increment: foo;
      content: counters(foo, ".");
    }

    @bottom-center {
      text-align: left;
      vertical-align: top;
      counter-reset: foo 2;
      counter-increment: foo foo foo;
      content: counters(foo, ".");
    }
  }
  @page :right {
    counter-reset: none;
    @bottom-center {
      counter-reset: inherit;
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
