# css/css-contain/content-visibility/content-visibility-auto-img-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-auto-img-001.html"
}
```

## style[0]

```css

body {
  /* body needs to be tall enough so that we can programmatically scroll.  Use
     'overflow:hidden' to suppress scrollbars so they don't interfere with the
     reftest snapshot. */
    height: 400vh;
    overflow: hidden;
}
img {
  width: 500px;  /* Much wider than the image's actual intrinsic width. */
  height: 210px; /* The image's actual intrinsic height. */
  object-fit: contain;
  object-position: 0 0;
  content-visibility: auto;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
