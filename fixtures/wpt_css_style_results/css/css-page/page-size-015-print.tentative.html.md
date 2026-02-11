# css/css-page/page-size-015-print.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-size-015-print.tentative.html"
}
```

## style[0]

```css

  /* The default page box size in WPT is 5 by 3 inches. */
  @page {
    width: 150vw;
    height: 200vh;
    margin: 0;

    /* This has no effect, since both width and height are specified,
       using viewport units. */
    size: 1234px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
