# css/css-values/viewport-units-scroll-no-cycles-001.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-values/viewport-units-scroll-no-cycles-001.tentative.html"
}
```

## style[0]

```css

  iframe {
    width: 100px;
    height: 100px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

@property --vw-tracker {
  syntax: '<length>';
  initial-value: 0px;
  inherits: false;
}

:root {
  --vw-tracker: 100vw;
  overflow: if(
    style(--vw-tracker = 100px): scroll;
    else: hidden;
  );
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
