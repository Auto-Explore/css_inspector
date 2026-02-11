# css/css-contain/content-visibility/content-visibility-forced-layout-client-rects.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-forced-layout-client-rects.html"
}
```

## style[0]

```css

body, html {
  padding: 0;
  margin: 0;
}
.spacer {
  height: 3000px;
}
.target {
  width: 12px;
  height: 34px;

  position: relative;
  left: 5px;
  top: 7px;
}

.hidden {
  content-visibility: hidden;
}

.will-hide {
  contain: style;
  contain: size;
  contain: layout;
  contain: paint;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
