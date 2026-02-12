# css/css-page/page-orientation-on-landscape-001-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-orientation-on-landscape-001-print.html"
}
```

## style[0]

```css


@page second-page {
  size: landscape;
  page-orientation: rotate-right;
}
div:nth-of-type(2) {
  page: second-page;
  break-before: page;

  box-sizing: border-box;
  width: 4in;
  height: 2in;
  border-top: 15px solid orange;
  border-right: none;
  border-bottom: 15px solid blue;
  border-left: none;
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
